# Tauri + React + Typescript + DDD

## Key Concepts of DDD in Rust
- **Entities**: Entities in Rust can be structs that implement an identifier to ensure their uniqueness.
- **Value Objects**: Value Objects are typically immutable and distinguished only by their attributes.
- **Aggregates**: Aggregates ensure consistency rules are enforced across entities and value objects.
- **Repositories**: Repositories encapsulate the logic for retrieving entities and aggregates. <br />
  A concrete implementation might use a database for persistence.
- **Services**: Services encapsulate domain logic that doesn’t naturally fit within an entity or value object.
- **Factories**: Factories help in creating complex objects.