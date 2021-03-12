- [Foreword](#foreword)
- [Motivation](#motivation)
- [Rust dev environment](#rust-dev-environment)
- [Compilation](#compilation)
- [Query filters](#query-filters)
- [QuerySet](#queryset)
- [Events](#events)
- [`system` chaining and code reuse](#-system--chaining-and-code-reuse)
- [Implementing game states](#implementing-game-states)
- [Rapier](#rapier)
- [Multi-platform support](#multi-platform-support)
- [Last part](#last-part)

### Foreword
[Rusty Bomber](https://github.com/rgripper/rusty-bomber) is a clone of the famous `BomberMan` game. It only remotely resembles original game and uses some nice open source assets [some art resources](https://github.com/rgripper/rusty-bomber#assets-and-attribution) from [opengameart.org](https://opengameart.org/).

### Motivation
I saw a post on reddit by [@rgripper](https://github.com/rgripper) looking for someone to help with a project using bevy engine. I got in touch with him and started working on it with learning by doing.

### Rust dev environment

We used the latest version of Rust (Bevy docs recommended the nightly version, which enables much faster compilation).
I coded in `vscode` + [`rust-analyzer`](https://github.com/rust-analyzer/rust-analyzer) (the latest release, I prefer to download source code and compile myself) + [`Tabline`](https://www.tabnine.com/) (optional) , alternatively you can use `Clion` + [`IntelliJ Rust`](https://www.jetbrains.com/rust/). 

### Compilation

Bevy’s web site mentions that the compilation is very fast.  And when version 0.4 was released, incremental compilation was even faster thanks to the dynamically linked feature, but it required a some extra configuration.

The compilation speed for Rust so far is in no way impressive, even annoying at times. But when working with Bevy, each incremental build is within acceptable limits if you have a decent laptop.

Here is my specs:
```
Intel(R) Core(TM) i5-8400 CPU @ 2.80GHz   2.81 GHz
RAM	16.0 GB
```
When the `dynamic` feature is enabled, each incremental compilation takes about 2.5 seconds. After adding other large dependencies, such as bevy_rapier, the incremental compilation speed will become longer, but still within acceptable limits, about 3.5 seconds. Five seconds is an acceptable amount for me to do a development iteration. During this development, the experience was great in terms of compile speed.

So how do you build a development environment that compiles quickly?

Here is a detailed explanation: https://bevyengine.org/learn/book/getting-started/setup/ (in the final section, Enable Fast Compiles (Optional))

You may stumble upon this puzzling error:

```shell
error: process didn't exit successfully: `target\debug\bevy_salamanders.exe` (exit code: 0xc0000139, STATUS_ENTRYPOINT_NOT_FOUND)
```

The solution is to change the line in the `.cargo/config.toml` File under the game project:

```toml
#before: 
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=y"]
#after:
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=off"]
```

If it doesn't help, try deleting `.cargo` folder and using only `dynamic` feature. Dynamic links are faster than switching a linker. For other problems I suggest you go to Bevy's official Discord channel or can submit an issue.

### Query filters

Bevy has a number of query filters built-in, and the 0.4 update is event easier to use and read.

Here’s how it works:

```rust
fn movement_system(
    query:Query<(/*Query components*/),(/*Query filter*/)>，
    mut example_query:Query<&mut Transform,With<Player>>
){
    for item in query.iter(){
        // Operate on the query results
    }
    for mut transform in example_query.iter_mut() {
        // Just use it like an iterator
    }
}
```

There are some basic examples [here](https://bevy-cheatbook.github.io/basics/queries.html#query-filters).

Common filters are `With<T>` , `Without<T>` , `Added<T>` , `Changed<T>` , `Mutated<T>` , `Or<T>` . `Added` only queries for newly added components, `Changed` is only for changes in existing components. `Mutated` is simply a mix of `Added` and `Changed`. `Or` is more special, and is used with other filters to reduce the scope of the query, but using `Or` can expand the scope of filtering. For example, to query the location and speed of players and creatures, you can define the query as:

```rust
    Query<(&Transform,&Speed),Or<(With<Player>,With<Creature>)>>
```
  
When a query has more than one component, it is necessary to use parentheses to pass the components in a tuple. 

Likewise, when a query has more than one filter, it is also necessary to use parentheses to pass the filter in a tuple.

When used with `Or`, it is usually used with `Option`. Such as querying both the position and speed of the player and the creature, as well as the player-specific component, the player’s power, you can write the query as follows:

```rust
    Query<(&Transform,&Speed,Option<&PlayerPower>),Or<(With<Player>,With<Creature>)>>
```

The resulting query with `Some(PlayerPower)` is definitely a `Player`, so treat it in the usual rust-like manner.

Example:
```rust
for (transform,speed,power) in query.iter() {
    if power.is_some() {
        // This means that the results of this iter belong Player.
    }
}
```
### QuerySet

When queries in a `system` conflict with each other, they cause a panic: `xxx has conflicting queries`. That’s where `QuerySet` is handy.

For example, here are two queries:

```rust
    mut q0: Query<(&Transform, &mut Point)>,
    q1: Query<&Transform, Or<(With<Point>, With<Head>)>>,
```

Both `Transform` and `Point` are queried, and `q1` contains the result of `q0`. But since `&mut` occur only once in the component of the query, there are no `&` other than `&mut`, so there is no conflict between the two queries.

Now take a look at the other two queries:

```rust
    mut q0: Query<(&mut Transform, &Point)>,
    q1: Query<&Transform, Or<(With<Point>, With<Head>)>>,
```

Here the first `Transform` component is declared with a `&mut` and in another - with `&`. This is where a query conflict occurs. To fix that we use `QuerySet`.

Consider the following two components:

```rust
pub struct Head;

pub struct Point {
    pub pre: Entity,
}
```

Suppose we need to write a system in which the position of each point changes in sync with the position of the previous entity:

```rust
fn position(
    mut q0: Query<(&mut Transform, &Point)>,
    q1: Query<&Transform, Or<(With<Point>, With<Head>)>>,
) {
    ...
}
```

We didn’t even implement any functionality for the system, and adding it directly to the App would have triggered query conflicts.
Now we replace the above query with `QuerySet`:

```rust
fn position(
    points_query: QuerySet<(
        Query<(&mut Transform, &Point)>,
        Query<&Transform, Or<(With<Point>, With<Head>)>>,
    )>,
) {
    ...
}
```

If you add it to the App without implementing anything, it will work. It’s also very easy to use, just pass the previous query as a tuple to `QuerSet` as a generic.

Without `QuerySet` our implementation would look like this:

```rust
fn position(
    q0: Query<(&mut Transform, &Point)>,
    q1: Query<&Transform, Or<(With<Point>, With<Head>)>>,
) {
    for (mut transform, point) in q0.iter_mut() } {
        if let Ok(pre_transform) = q1.get(point.pre)  {
            *transform = Transform::from_translation(
                pre_transform.translation - Vec3::new(1.0, 1.0, 0.0)
            );
        }
    }
}
```

And with `QuerySet`:

```rust
fn position(
    mut points_query: QuerySet<(
        Query<(&mut Transform, &Point)>,
        Query<&Transform, Or<(With<Point>, With<Head>)>>,
    )>,
) {
    for (mut transform, point) in points_query.q0_mut().iter_mut() {
        if let Ok(pre_transform) = points_query.q1().get(point.pre) {
            *transform = Transform::from_translation(
                pre_transform.translation - Vec3::new(1.0, 1.0, 0.0) * 30.0,
            )
        } else {
            warn!("Not the right transform!");
        }
    }  
}
```

Before we run our code, `rust-analyzer` reported an error. We passed the `&mut points_query` in `q0_mut()` , and according to the borrowing check, the pointer of `points_query` can no longer be borrowed, so here we need to use `unsafe`. But before that we should make sure that our `unsafe` call is actually safe.

Looking at the documentation for the `iter_unsafe()` we'll be calling, you can see the `Safety` hint:

> This allows aliased mutability. You must make sure this call does not result in multiple mutable references to the same component

We are sure and the code is safe!

Now our code looks like this:

```rust
fn position(
    mut points_query: QuerySet<(
        Query<(&mut Transform, &Point)>,
        Query<&Transform, Or<(With<Point>, With<Head>)>>,
    )>,
) {
    // Safety: It is necessary to write Safety in actual production
    for (mut transform, point) in unsafe { points_query.q0().iter_unsafe() } {
        if let Ok(pre_transform) = points_query.q1().get(point.pre)  {
            *transform = Transform::from_translation(
                pre_transform.translation - Vec3::new(1.0, 1.0, 0.0) * 30.0,
            )
        } else {
            warn!("Not the right transform!");
        }
    }
}
```


Once you resolve the borrowing error reported by rust-analyzer, run the App again, and you can see the results of the query.

Talking about the `QuerySet` experience, since `rust-analyzer` wasn’t very friendly to the API support generated by process macros, the code complement experience for macro-generated API was pretty bad. Of course it's not the API's problem, it's our IDE ecology problem, lol.

### Events

In Bevy 0.4 the `EventReader` is not a real iterator and requires an event reference when calling `iter()`:

```rust
pub fn game_events_handle(
    game_events: Res<Events<GameEvents>>,
    mut events_reader: Local<EventReader<GameEvents>>,
) -> Result<()> {
   for event in events_reader.iter(&game_events) {
        match event {
            ...
        }
    }
}
```

Fortunately, in the upcoming 0.5 release (after this [PR](https://github.com/bevyengine/bevy/pull/1244)) `iter` call will not need any arguments:

```rust
pub fn game_events_handle(
    mut events_reader: EventReader<GameEvents>,
) -> Result<()> {
    for event in events_reader.iter() {
        match event {
            ...
        }
    }
}
```

### `system` chaining and code reuse

There was a system example in the Events section that was different from the other general examples:

```rust
pub fn game_events_handle(
    mut events_reader: EventReader<GameEvents>,
) -> Result<()> {
    ...
}
```

Its return value is a `Result` and if you add the system directly to the `App`, it will be reported as an error by `rust-analyzer`. That's because bevy does not support systems with a return value. So how do you add a system with a return value to the `App`? Bevy provides us with a `fn chain(self, system: SystemB)` function that can be used like this:

```rust
    .add_system(game_events_handle.system().chain(error_handler.system()))
```

You can chain as many systems as you like. Chaining is explained [here](https://bevy-cheatbook.github.io/basics/system-chaining.html#system-chaining). You can also do this:

```rust
pub fn body_point_translation_handle(
    come_in: In<Option<Vec3>>,
    mut query: Query<&mut Transform, With<BodyPoint>>,
) {
	...
}
```

Yes, you can also add new parameters to each chain node. This chaining is one of my favorite bevy features that is both practical and flexible.
Asynchronous chains are also possible (look at this [PR](https://github.com/bevyengine/bevy/pull/1393)).

### Implementing game states

We implemented a complete game flow in our project, including a menu UI for starting the game, a pause inside the game, and a Gave Over UI when the player is killed by a bomb or touched by a creature, and Victory UI when the player finds the entrance to the next level. This game has just enough features to help learn Bevy and does not really have proper levels or different enemies. To implement the game flow I referred to [`Kataster`](https://github.com/Bobox214/Kataster)’s code and put all the possible game states in the body of `AppState` enum:

```rust
pub enum AppState {
    StartMenu,
    Game
}
```

Building the game state typically consists of these:

1. Add game state as a resource:

    ```rust
    app.add_resource(State::new(AppState::StartMenu))
    // We need to point out the state of the game when we initialize it.
    ```

2. Initialize `StateStage`:

   ```rust
   // The code that links up the first step
       .add_stage_after(// It’s also flexible enough to do what you like
           stage::UPDATE,// target, you can put your state in any existing state you want
           APP_STATE_STAGE,// name, the name is also flexible, you can choose your own name, here is const APP_STATE_STAGE: &str = "app_state";
           StateStage::<AppState>::default(),// Here you need to initialize your enumerate of game's state as a generic of StateStage.
   )
   ```

3. Handle different states
    ```rust
    // The code that links up the pre step
    .stage(APP_STATE_STAGE, |stage: &mut StateStage<AppState>| {
            // With this closure, we can add systems to the different states of our game
            stage
                // start menu
                // on_state_enter: This is usually used to add the load resource system for the game enter at this stage.
                .on_state_enter(AppState::StartMenu, start_menu.system())
                // on_state_update: This is usually used to add the logical system for the game update at this stage.
                .on_state_update(AppState::StartMenu, button_system.system())
                // on_state_exit: This is usually used to clear the next stage of the game does not need resources.
                .on_state_exit(AppState::StartMenu, exit_ui_despawn.system())
                // in game
                .on_state_enter(AppState::Game, setup_map.system()))
                // Similar to on_state_update, but you can set more than one at one time
                .update_stage(AppState::Game, |stage: &mut SystemStage| {
                    stage
                    // None of the following methods came with SystemStage itself, but were implemented under the modules of our game project through custom trait to the SystemStage, just for the convenience of managing the modules.
                        .physics_systems()
                        .player_systems()
                        .bomb_systems()
                        .buff_systems()
                        .creature_systems()
                        .portal_systems()
                })
                .on_state_exit(AppState::Game, exit_game_despawn.system())
        });
    ```

4. Map state machine transitions

    ```rust
    // In addition, build a system to handle the jump of game state
    pub fn jump_state(
        mut app_state: ResMut<State<AppState>>,
        input: Res<Input<KeyCode>>,
        mut app_exit_events: ResMut<Events<AppExit>>,
    ) -> Result<()> {
        // Using pattern matching makes it very clear that our game state transitions will be handled
        match app_state.current() {
            AppState::StartMenu => {
                if input.just_pressed(KeyCode::Return) {
                    // The method set_next is a jump from the current state to the specified state
                    app_state.set_next(AppState::Game)?;
                }
                if input.just_pressed(KeyCode::Escape) {
                    app_exit_events.send(AppExit);
                }
            }
            AppState::Game => {
                if input.just_pressed(KeyCode::Back) {
                    app_state.set_next(AppState::StartMenu)?;
                    map.init();
                }
            }
        }
        Ok(())
    }
    ```

Let’s talk about step three, which will probably be replaced by a [new scheduler](https://github.com/bevyengine/bevy/pull/1144) in later versions, but that’s far away in the future, and until then, we need new blogs which someone will write about the issue. 


### Rapier

`Rapier` is a powerful physics engine, and this project barely touched it. You can learn more about `rapier` in the [official documentation](https://rapier.rs/docs/user_guides/rust/getting_started) and in `bevy_rapier` group on [discord](https://discord.gg/VuvMUaxh) to learn more.

Two common components of `Rapier` are `RigidBody` and `Collider`. Each entity in Bevy can only have one rigid body, while a collider can have multiple, such as a character’s head, arms, and legs.

The way to create a rigid body is simple:

```rust
// To create a kinematic rigidbody
RigidBodyBuilder::new_kinematic()
.translation(translation_x, translation_y)
// To create a static rigidbody
RigidBodyBuilder::new_static()
.translation(translation_x, translation_y)        
// To create a dynamic rigidbody
RigidBodyBuilder::new_dynamic()        
.translation(translation_x, translation_y)        
.lock_rotations()// (optional) lock the rigid body for rotation    
.lock_translations()// (optional) lock the rigid body for translation
```

To create a `RigidBody`, you need to specify its translation. Like this:

```rust
RigidBodyBuilder::new_dynamic()        
.translation(translation_x, translation_y)
...// more other operation  
```

Since `bevy_rapier` has a system for sync `RigidBody`’s transform(this is not a component, it's a rigidbody field) to the entity’s `Transform`(a bevy component). 

Which means we no longer need to manage the entity’s `Transform`(bevy component), only through the rigidbody to manage the entity’s speed, position, rotation, force, and so on.

Here we create a collider:

```rust
// Rapier has a number of options, and since we only use two of them in our game projects, we’re only going to talk about these two categories
// A cuboid, set to provide its half height and half width
ColliderBuilder::cuboid(hx, hy)
// a ball, set to provide a radius
ColliderBuilder::ball(radius)
```

> The parameters for building a cuboid collider are half height and half width, not full height and full width.


For a single collider with a rigid body, they can be directly inserted as components into an existing entity.

But for multiple colliders with a rigid body is slightly different, see [here](https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier2d/examples/multiple_colliders2.rs).

```rust
fn for_player_add_collision_detection(
    commands: &mut Commands,
    query: Query<
        (Entity, &Transform),
        (
            With<Player>,
            Without<RigidBodyBuilder>,
            Without<ColliderBuilder>,
            Without<RigidBodyHandleComponent>,
            Without<ColliderHandleComponent>,
        ),
    >,
) {
    for (entity, transform) in query.iter() {
        let translation = transform.translation;
        commands.insert(
            entity,
            (
                create_dyn_rigid_body(translation.x, translation.y),
                create_player_collider(entity),
            ),
        );
    }
}
```

After all map resources are loaded, we insert the corresponding rigid body and collider into the entity without rigid body and collision body.

Adding these isn’t enough to get the physics engine running in our game, mainly because `bevy_rapier` is still being imported as an external crate. Right now you have to manually add this plugin:

```rust
    app
        .add_plugin(RapierPhysicsPlugin)
```

With the physics engine successfully enabled in our game, we need to allow creatures collide with each other:

```rust
    ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)
        // You can insert some data that you need to save, but only in u128 format, which is usually used to store Entity
        .user_data(entity.to_bits() as u128)
        // You can set up an interaction group to allow the collider to interact with the set interaction group
        .solver_groups(InteractionGroups::new(WAY_GROUPS, NONE_GROUPS))
        // After the interaction group is also set up, the collider is allowed to solve the collision under the rules of the group
        .collision_groups(InteractionGroups::new(WAY_GROUPS, NONE_GROUPS))
```

Before going any further into the difference between a solver group and a collision group, we need to understand the rules for constructing an interaction group. We need two parameters for the interaction group's `new()`, the first (`u16`) - to specify which group the collider belongs to, and the second (`u16`) - the collider and which a groups of Colliders will interact.

For the second parameter, it’s easy to understand how settings interact with a single collider, but how about multiple colliders? This is the beauty of setting the parameter type to `u16`, for example:

```rust
const CREATURE_GROUPS: u16 = 0b0010;
const PLAYER_GROUPS: u16 = 0b0001;
const WALL_GROUPS: u16 = 0b0100;
const WAY_GROUPS: u16 = 0b1000;
const NONE_GROUPS: u16 = 0b0000;
```

The difference between a solution group and a collision group: 

If a group is a solution group, then it should participate in all force analysis; 

If a group is a collision group, then it should participate in all collision events, these collision events are received and processed through `Res<eventqueue>`.

And `user_data`, which is passed into the collider builder when the collider is inserted, can be used to obtain the entity using the following method:

```rust
let entity = Entity::from_bits(user_data as u64);
```

Where does `user_data` come from? From the collision event we get an index that can be accessed via the `Res<ColliderSet>` 's `get()` method user, which is cumbersome and I think the least usable part of `bevy_rapier` so far.

In addition, if you run the game from here, you’ll find that your character, as well as the other dynamic rigid bodies on the screen, will receive a gravitational force. We don’t need this gravity, so we need to change it to zero.

Ccurrent version modifies the gravity by adding a startup system like this:

```rust
fn setup(
    mut configuration: ResMut<RapierConfiguration>,
) {
    configuration.gravity = Vector::y() * 0.0;
}
```

This system is added to `startup_system()`, and only run once.

### Multi-platform support

Our game now runs on WASM as well as usual desktop platforms. Since Bevy’s doesn't support audio in WASM version there is no audio in our game. Bevy’s support platforms includes mobile, and while there are few changes to port to mobile, we decided to stick with WASM.

The bevy rendering backend uses the WGPU, although the WGPU rendering backend supports WASM, for some reason it didn't work with Bevy, the bevy WASM project we have been able to refer to is basically based on `bevy_webgl2`.

To support WASM you need to also put these:

```rust
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(bevy_webgl2::DefaultPlugins);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins);
```

Turn off the `bevy_wgpu` feature:

```toml
[features]
# This part is the feature of bevy that both native and wasm will use
default = [
  "bevy/bevy_gltf",
  "bevy/bevy_winit",
  "bevy/bevy_gilrs",
  "bevy/render",
  "bevy/png",
]
# This part is the feature that native will use
native = [
  "bevy/bevy_wgpu",
  "bevy/dynamic"# （Optional, increase the incremental compilation speed during development, the compilation is really fast!）
]
# This part is the webgl2 feature that wasm support will use
web = [
  "bevy_webgl2"
]
```

Supporting web requires tinkering with HTML, `cargo make` and we even used a js/wasm bundler to deploy on Github Pages.
There are official examples for use on Android, (see [`cargo mobile`](https://github.com/BrainiumLLC/cargo-mobile)’s README for details).
### Last part

Many thanks to Rapier author [@Sébastien Crozet](https://github.com/sebcrozet). I’m using the physics engine for the first time, and there are a lot of things I don’t understand that were kindly provided by people in the discord group.

I would also like to thank my coding buddy [@rgripper](https://github.com/rgripper). Without our cooperation, this project would not have been possible.
I enjoyed coding with him and we learned a lot together on this project. If you want to learn about Bevy through practice, feel free to contact us and maybe we code our next game with you.
