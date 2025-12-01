# MVC Architecture in Rust: A Comprehensive Guide

## Overview of MVC in Rust

MVC (Model-View-Controller) in Rust looks quite different from classical object-oriented implementations. Rust's philosophy of composition over inheritance means you'll use structs, traits, and modules rather than class hierarchies.

## The Rust MVC Pattern

### Model Layer
The Model represents your data and business logic. In Rust, this is typically:

```rust
// models/user.rs
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, username: String, email: String) -> Self {
        User { id, username, email }
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        Ok(())
    }
}
```

### View Layer
Views handle presentation. In web frameworks, this might be:
- Templates (Tera, Handlebars, Askama)
- JSON serialization (serde_json)
- HTML rendering

```rust
// views/user_view.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct UserView {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        UserView {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}
```

### Controller Layer
Controllers coordinate between Models and Views:

```rust
// controllers/user_controller.rs
pub struct UserController {
    user_repository: UserRepository,
}

impl UserController {
    pub fn new(user_repository: UserRepository) -> Self {
        UserController { user_repository }
    }
    
    pub async fn get_user(&self, id: u32) -> Result<UserView, Error> {
        let user = self.user_repository.find_by_id(id).await?;
        Ok(UserView::from(user))
    }
    
    pub async fn create_user(&self, data: CreateUserDto) -> Result<UserView, Error> {
        let user = User::new(generate_id(), data.username, data.email);
        user.validate()?;
        let saved_user = self.user_repository.save(user).await?;
        Ok(UserView::from(saved_user))
    }
}
```

## Typical Rust Project Structure

```
my_project/
├── src/
│   ├── main.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── post.rs
│   ├── views/
│   │   ├── mod.rs
│   │   └── user_view.rs
│   ├── controllers/
│   │   ├── mod.rs
│   │   └── user_controller.rs
│   ├── repositories/
│   │   ├── mod.rs
│   │   └── user_repository.rs
│   └── routes/
│       ├── mod.rs
│       └── user_routes.rs
├── Cargo.toml
└── .gitignore
```

## How Rust Replaces Inheritance and Classes

### 1. Composition Over Inheritance

Instead of class hierarchies, Rust uses **struct composition**:

```rust
// Instead of inheritance, use composition
struct Engine {
    horsepower: u32,
}

struct Car {
    engine: Engine,  // Composition
    model: String,
}
```

### 2. Traits for Polymorphism

Traits replace interfaces and abstract classes:

```rust
// Define shared behavior
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// Implement the trait for different types
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

// Use trait objects for runtime polymorphism
fn render(shape: &dyn Drawable) {
    shape.draw();
}
```

### 3. Trait Bounds for Generic Programming

```rust
// Generic function with trait bounds
fn process<T: Drawable + Clone>(item: &T) {
    item.draw();
}

// Multiple trait bounds
trait Serialize {
    fn to_json(&self) -> String;
}

fn save_to_db<T: Drawable + Serialize>(item: &T) {
    let json = item.to_json();
    // Save to database
}
```

### 4. Enums for Type Variants

Rust's enums are powerful for representing different variants of data:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

### 5. Trait Objects for Dynamic Dispatch

```rust
// Store different types that implement the same trait
struct Canvas {
    shapes: Vec<Box<dyn Drawable>>,
}

impl Canvas {
    fn render_all(&self) {
        for shape in &self.shapes {
            shape.draw();
        }
    }
}
```

### 6. Associated Types and Generic Traits

```rust
trait Repository {
    type Entity;
    type Id;
    
    fn find_by_id(&self, id: Self::Id) -> Option<Self::Entity>;
    fn save(&mut self, entity: Self::Entity) -> Self::Id;
}

struct UserRepository;

impl Repository for UserRepository {
    type Entity = User;
    type Id = u32;
    
    fn find_by_id(&self, id: u32) -> Option<User> {
        // Implementation
        None
    }
    
    fn save(&mut self, user: User) -> u32 {
        // Implementation
        0
    }
}
```

## Real-World MVC Example with Axum

```rust
use axum::{
    Router,
    routing::{get, post},
    extract::{Path, State},
    Json,
};

// Model
#[derive(Clone)]
struct User {
    id: u32,
    username: String,
}

// Repository (Model layer)
struct UserRepository {
    // Database connection would go here
}

impl UserRepository {
    async fn find_by_id(&self, id: u32) -> Option<User> {
        // Database query
        None
    }
}

// Controller
struct AppState {
    user_repo: UserRepository,
}

async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Json<User> {
    let user = state.user_repo.find_by_id(id).await.unwrap();
    Json(user)
}

// Routes (ties it together)
fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users/:id", get(get_user_handler))
        .with_state(state)
}
```

## Key Takeaways

1. **MVC in Rust is modular**: Use modules and structs rather than class hierarchies
2. **Traits replace inheritance**: Define shared behavior through trait implementation
3. **Composition is preferred**: Build complex types by combining simpler ones
4. **Enums are powerful**: Use them for type variants and state machines
5. **Ownership matters**: The borrow checker influences architecture decisions
6. **Zero-cost abstractions**: Traits and generics compile to efficient code

## Popular Rust Web Frameworks Using MVC

- **Axum**: Lightweight, uses extractors and handlers
- **Actix-web**: Actor-based, high performance
- **Rocket**: Rails-like, macro-heavy
- **Warp**: Filter-based routing

Each implements MVC concepts differently, but all leverage Rust's type system for safety and performance.