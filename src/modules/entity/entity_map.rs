use std::vec::Vec;
use std::cmp::max;
use std::mem::replace;

/// Attempting to use Generational Index to implement the backbone of the SoA for Systems to query
/// 
/// Each Element has a "Generation" and Index
/// Whenever an allocation occurs increment the Index
/// Whenever a deallocation occurs reset the Index and increment the "Generation"
/// 
/// This is essentially an Associative Array DS
/// 
/// In generational_arena it appears only data on the same generation is returned, it appears this is
/// how the DS works to maintain synchronicity
/// 
/// Shamelessly stolen from https://github.com/fitzgen/generational-arena/blob/master/src/lib.rs
/// 

#[derive(Clone, Debug)]
pub struct EntityMap<T> {
    items: Vec<EntityEntry<T>>,
    generational_index: u64,
    free_list_head: Option<usize>,
    len: usize,
}

/// This is the Index
#[derive(Clone, Debug)]
pub struct Entity {
    index: usize,
    generational_index: u64,
}

#[derive(Clone,Debug)]
enum EntityEntry<T> {
    Free { next_free: Option<usize> },
    Occupied { generational_index: u64, value: T },
}

impl Entity {
    pub fn new(a: usize, b: u64) -> Entity {
        Entity {
            index: a,
            generational_index: b
        }
    }

    pub fn print(&self) {
        println!("Index: {} | Generation: {}", self.index, self.generational_index)
    }
}

const DEFAULT_CAPACITY: usize = 4;

impl<T> Default for EntityMap<T> {
    fn default() -> EntityMap<T> {
        EntityMap::new()
    }
}

impl<T> EntityMap<T> {
    pub fn new() -> EntityMap<T> {
        EntityMap::with_capcity(DEFAULT_CAPACITY)
    }

    pub fn with_capcity(n: usize) -> EntityMap<T> {
        let n = max(n, 1);
        let mut entity_map = EntityMap {
            items: Vec::new(),
            generational_index: 0,
            free_list_head: None,
            len: 0,
        };
        entity_map.reserve(n);
        entity_map
    }

    #[inline]
    pub fn try_insert(&mut self, value: T) -> Result<Entity, T> {
        match self.try_alloc_next_index() {
            None => Err(value),
            Some(index) => {
                self.items[index.index] = EntityEntry::Occupied {
                    generational_index: self.generational_index,
                    value
                };
                Ok(index)
            }
        }
    }

    #[inline]
    pub fn try_alloc_next_index(&mut self) -> Option<Entity> {
        match self.free_list_head {
            None => None,
            Some(i) => match self.items[i] {
                EntityEntry::Occupied { .. } => panic!("corrupt free list"),
                EntityEntry::Free { next_free } => {
                    self.free_list_head = next_free;
                    self.len += 1;
                    Some(Entity {
                        index: i,
                        generational_index: self.generational_index
                    })
                }
            }
        }
    }

    #[inline]
    pub fn insert(&mut self, value: T) -> Entity {
        match self.try_insert(value) {
            Ok(i) => i,
            Err(value) => self.insert_slow_path(value)
        }
    }

    #[inline]
    pub fn insert_slow_path(&mut self, value: T) -> Entity {
        let len = if self.capacity() == 0 {
            1
        } else {
            self.items.len()
        };
        self.reserve(len);
        self.try_insert(value)
            .map_err(|_| ())
            .expect("inserting will always succeed after reserving additional space")
    }

    pub fn remove(&mut self, i: Entity) -> Option<T> {
        if i.index >= self.items.len() {
            return None;
        }

        match self.items[i.index] {
            EntityEntry::Occupied { generational_index, .. } if i.generational_index == generational_index => {
                let entry = replace(&mut self.items[i.index], EntityEntry::Free { next_free: self.free_list_head });
                self.generational_index += 1;
                self.free_list_head = Some(i.index);
                self.len -= 1;

                match entry {
                    EntityEntry::Occupied { generational_index: _, value } => Some(value),
                    _ => unreachable!(),
                }
            }
            _ => None,
        }
    }

    pub fn capacity(&self) -> usize {
        self.items.len()
    }

    pub fn contains(&self, i: Entity) -> bool {
        self.get(i).is_some()
    }

    pub fn get(&self, i: Entity) -> Option<&T> {
        match self.items.get(i.index) {
            Some(EntityEntry::Occupied {
                generational_index,
                value,
            }) if *generational_index == i.generational_index => Some(value),
            _ => None
        }
    }

    pub fn get_mut(&mut self, i: Entity) -> Option<&mut T> {
        match self.items.get_mut(i.index) {
            Some(EntityEntry::Occupied {
                generational_index,
                value
            }) if *generational_index == i.generational_index => Some(value),
            _ => None
        }
    }

    pub fn reserve(&mut self, additional_capacity: usize) {
        let start = self.items.len();
        let end = self.items.len() + additional_capacity;
        let old_head = self.free_list_head;
        self.items.reserve_exact(additional_capacity);
        self.items.extend((start..end).map(|i| {
            if i == end - 1 {
                EntityEntry::Free {
                    next_free: old_head,
                }
            } else {
                EntityEntry::Free {
                    next_free: Some(i + 1),
                }
            }
        }));
        self.free_list_head = Some(start);
    }
}
