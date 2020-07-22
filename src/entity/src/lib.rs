use uuid::Uuid;

pub struct Entity {
    tag: Uuid,
}

impl Entity {
    fn new() -> Entity {
        let uuid4 = Uuid::new_v4();
        Entity { tag: uuid4 }
    }
}

pub fn generate_entity() -> Entity {
    Entity::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entity_create() {
        let entity = generate_entity();
        assert_eq!(entity.tag.is_nil(), false)
    }
}
