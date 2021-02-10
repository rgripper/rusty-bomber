### Foreword

[Rusty BomberMan](https://github.com/rgripper/rusty-bomber) is a bevy of the famous `BomberMan` game. Although it is a remake version, it actually looks completely different from the original game. The reason is that the art resources of the original game are not available, so we found [some art resources](https://github.com/rgripper/rusty-bomber#assets-and-attribution) form [opengameart.org](https://opengameart.org/). Thank you very much for these resources.

### Motivation for development

The reason for developing this game was that I was visiting reddit , and I saw a post by [@rgripper](https://github.com/rgripper) looking for someone to work on the bevy project together. I got in touch with him and started working on it with a mindset of learning and doing.

### Rust development environment recommendation

Use the latest version of Rust in development (recommended nightly version , this seems to be useful for quick compilation).

The development environment recommends `vscode` + [`rust-analyzer`](https://github.com/rust-analyzer/rust-analyzer) (install the latest release, I prefer to download source code and compile it myself.) + [`Tabline`](https://www.tabnine.com/) (optional) , or `Clion` + [`IntelliJ Rust`](https://www.jetbrains.com/rust/). 

### Compile speed

Bevy’s web site mentions that the compilation is very fast.  And when version 0.4 was released, incremental compilation was much faster thanks to the dynamically linked feature, but it required a number of configurations.

The compilation speed of rust itself isn't fast, or even annoying at times, but during the bevy development iteration, each incremental build is within acceptable limits if you have a well-configured development environment for fast compilation.

The configuration of my laptop is:

```
Intel(R) Core(TM) i5-8400 CPU @ 2.80GHz   2.81 GHz
RAM	16.0 GB
```

When the `dynamic` feature is enabled for compilation, each incremental compilation takes about 2.5 seconds. After adding other large dependencies, such as bevy_rapier, the incremental compilation speed will become longer, but still within acceptable limits, about 3.5 seconds. Five seconds is an acceptable amount for me to do a development iteration. During this development, the experience was great in terms of compile speed.

So how do you build a development environment that compiles quickly?

The site explains in detail how to set up a fast development environment: https://bevyengine.org/learn/book/getting-started/setup/(in the final section, Enable Fast Compiles (Optional))

There may be some strange questions in setting up the environment, such as this:

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

After the change if there are similar strange mistakes, you can try to delete directly `.cargo` this folder , using only `dynamic` feature, dynamic links to compile speed is much greater than switching linker. And any other weird, unresolved issues, then you can submit an issue.

### Query filter

Bevy has a number of query filters built in, and the 0.4 update has made it easier to use and read.

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

You can see the general usage [here](https://bevy-cheatbook.github.io/basics/queries.html#query-filters).

Common filters are `With<T>` , `Without<T>` , `Added<T>` , `Changed<T>` , `Mutated<T>` , `Or<T>` , where `Mutated` is a collection of `Added` and `Changed`, and `Added` only queries for newly added components, changed to query only `Changed` components in existing components, `Or` is more special, the use of several other filters are basically to reduce the scope of the query, but using `Or` can expand the scope of filtering, for example, to query the location and speed of players and creatures, you can define the query as:

```rust
    Query<(&Transform,&Speed),Or<(With<Player>,With<Creature>)>>
```

When a query has more than one component, it is necessary to use parentheses to pass more than one component as a tuple. Likewise, many filters pass parameters as tuples. Using `Or`, of course, is often used with `Option`, such as querying both the position and speed of the player and the creature, as well as the player specific component, the player’s power, you can write the query as follows:

```rust
    Query<(&Transform,&Speed,Option<&PlayerPower>),Or<(With<Player>,With<Creature>)>>
```

The resulting query with `Some(PlayerPower)` is definitely a `Player`, so treat it in the usual rust-like manner.

### QuerySet

When queries in a `system` conflict with each other, a compiled run triggers a panic: `xxx has conflicting queries`. That’s where `QuerySet` comes in.

For example, here are two queries:

```rust
    mut q0: Query<(&Transform, &mut Point)>,
    q1: Query<&Transform, Or<(With<Point>, With<Head>)>>,
```

Both `Transform` and `Point` are queried, and `q1` contains the result of `q0`, but since `&mut` occur only once in the component of the query, there are no `&` other than `&mut`, so there is no conflict between the two queries.

Take a look at the following two queries:

```rust
    mut q0: Query<(&mut Transform, &Point)>,
    q1: Query<&Transform, Or<(With<Point>, With<Head>)>>,
```

Similar to the example above, but the difference is that the `Transform` component has one with `&mut` and the other with `&`, this is where a query conflict occurs.

After query conflicts, that’s where `QuerySet` comes in.

Consider the following two components:

```rust
pub struct Head;

pub struct Point {
    pub pre: Entity,
}
```

Suppose we need to write a system in which the position of each point changes according to the position of the previous entity:

```rust
fn position(
    mut q0: Query<(&mut Transform, &Point)>,
    q1: Query<&Transform, Or<(With<Point>, With<Head>)>>,
) {
    ...
}
```

We didn’t even implement any functionality for the system, and adding it directly to the App would have triggered query conflicts.

Replace the above query with `QuerySet`:

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

What about the implementation? Without `QuerySet`, our implementation would look something like this:

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

So with `QuerySet`, our content should look something like this:

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
            warn!("Not find right transform!");
        }
    }  
}
```

Before we run our code, `rust-analyzer` reported an error. We passed the `&mut points_query` in `q0_mut()` , and according to the borrowing check, the pointer of `points_query` can no longer be borrowed, so here we need to use unsafe. But before we use `unsafe`, we should make sure that our `unsafe` call is safe.

Looking at the documentation for the `iter_unsafe()` we'll be calling, you can see the `Safety` hint:

> This allows aliased mutability. You must make sure this call does not result in multiple mutable references to the same component

After analyzing our query, we were able to make sure this call does not result in multiple mutable references to the same component, so calling the unsafe function here is safe!

After adding unsafe, our code looks like this:

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
            warn!("not find right transform!");
        }
    }
}
```

Once you resolve the borrowing error reported by `rust-analyzer`, run our `App` again, and you’ll get what you want.

Talking about the `QuerySet` experience, since `rust-analyzer` wasn’t very friendly to the API support generated by process macros, the code complement experience for macro-generated API was pretty bad. Of course it's not the API's problem, it's our IDE ecology problem, lol.

### Events

The 0.4 bevy event has one major drawback, as shown in the following example:

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

The `EventReader` is not a real iterator and needs to pass a event reference when calling `iter()` , which feels redundant during use.

Fortunately, `EventReader` has been improved in the upcoming 0.5 release, and after this [PR](https://github.com/bevyengine/bevy/pull/1244) merge, the `EventReader` call has gone like this:

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

It is important to note that not only has `EventReader` become a higher-level API (that is, a real system parameter) , but `Events` become a higher-level API, eliminating the need for a layer of `ResMut` on the outside.

### `system` chaining and code reuse

There was a system example in the Events section that was different from the other general examples:

```rust
pub fn game_events_handle(
    mut events_reader: EventReader<GameEvents>,
) -> Result<()> {
    ...
}
```

It has a `Result` return value, and if you add the system directly to the `App`, it will be reported as an error by `rust-analyzer` because bevy does not support systems with a return value. So how do you add a system with a return value to the `App`? To get rid of its return value, bevy provides us with a `fn chain(self, system: SystemB)` function that calls something like this:

```rust
    .add_system(game_events_handle.system().chain(error_handler.system()))
```

It can be ‘Unlimited refills’ and you can go on forever if you want. How do you write a chain system? You can see [here](https://bevy-cheatbook.github.io/basics/system-chaining.html#system-chaining). In addition to the usage shown here, you can even use:

```rust
pub fn body_point_translation_handle(
    come_in: In<Option<Vec3>>,
    mut query: Query<&mut Transform, With<BodyPoint>>,
) {
	...
}
```

Yes, you can add new parameters to each chain node, which greatly increases code flexibility and code reuse. This is one of my favorite bevy features that is both practical and flexible. Although in this project not much use, basically used to do error handling, but I believe that in a large project, this function can fully play its advantages, it’s probably because bevy is full of ergonomics designed like this that everyone’s excited about it.

Of course, the current system is not perfect, to deal with certain situations we will probably need an asynchronous chain, fortunately, now has the [PR](https://github.com/bevyengine/bevy/pull/1393).

### How to implement different states of  the game

We implemented a complete game flow in our project, including a menu interface for starting the game, a pause inside the game, and a failure when a player is killed by a bomb or touched by a creature, and the victory when the player finds the entrance to the next level. If you have experienced our game, you will find that the level is not designed, but only to achieve the effects of various props in the game, including the first level and the second level of the difference is only a few more creature. As a game, I’m not happy with this implementation, but as an experience, as a learning bevy, I’ve learned a lot. I even kept a random level implementation interface, but didn’t actually implement it. I had no prior experience with Roguelike’s algorithms, and I was just hoping that the next project would improve on that.

To get back to the point, in order to implement such a complete game flow, I refer to [`Kataster`](https://github.com/Bobox214/Kataster)’s code and put the whole game flow in the body of the `AppState` enumeration:

```rust
pub enum AppState {
    StartMenu,
    Game
}
```

Building the state of a game typically requires the following four steps:

1. Add our game state as a resource to the game:

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

3. Processing stage

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

4. Handle game state transitions

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

With these four steps, you can add different states to your game. Now let’s talk about step three, which will probably be replaced by a [new scheduler](https://github.com/bevyengine/bevy/pull/1144) in later versions, but that’s a long time away, until then, new blogs are needed.

### Rapier

`Rapier` is a very rich physical engine, and the content of this project is only a small part of it, and this article has only selected some meaningful records from it. If you want to learn more about `rapier`, my advice is to read the [official documentation](https://rapier.rs/docs/user_guides/rust/getting_started) first and then go to the `bevy_rapier` group in [discord](https://discord.gg/VuvMUaxh) to learn more.

Two common components of `Rapier` are the `RigidBody` and the `Collider`. Each entity in bevy can have only one rigid body, while a collider can have multiple, such as a character’s head, arms, and legs, all of which can be represented by a single collider.

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

> To create a `RigidBody`, you need to specify its location, because within `bevy_rapier` there is a system for transforming the `RigidBody`’s location and the entity’s `Transform`, which means we no longer need to manage the entity’s `Transform`, only through the rigid body to manage the entity’s speed, position, rotation, force, and so on.

The way to create colliders is also simple:

```rust
// Rapier has a number of options, and since we only use two of them in our game projects, we’re only going to talk about these two categories
// A cuboid, set to provide its half height and half width
ColliderBuilder::cuboid(hx, hy)
// a ball, set to provide a radius
ColliderBuilder::ball(radius)
```

> Note: the required parameters for building a cuboid collider are half height and half width, not full height and full width.

For a single collider, a rigid body and a collider can be directly inserted as a component into an existing entity(The method of adding multiple colliders to a rigid body is slightly different from the method of adding a single collider to a rigid body, see [here](https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier2d/examples/multiple_colliders2.rs).):

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

In our game, we use final loading, that is, after all map resources are loaded, we insert the corresponding rigid body and collider into the entity without rigid body and collision body.

The last two filters, `Without<RigidBodyHandleComponent>`and `Without<ColliderHandleComponent>`, are actually because bevy has a system inside that transforms the `Builder` into a `HandleComponent`. When we insert the builder into the entity, the system then converts it into a handle component in some internal way. So in order to prevent our query results have been inserted in the handle component of the entity, so we need to add this filter.

Adding these isn’t enough to get the physics engine running in our game, mainly because `bevy_rapier` is still being imported as an external crate, in the future, if `bevy_rapier` integrate into the bevy’s physical engine, you won’t need to do this:

```rust
    app
        .add_plugin(RapierPhysicsPlugin)
```

With this simple setup, the physics engine was successfully enabled in our game. One thing in particular to note is that in our game, creatures can collide with each other, so how do we do that? Just Point to the solution group or collision group when creating the collider.

```rust
    ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)
        // You can insert some data that you need to save, but only in u128 format, which is usually used to store Entity
        .user_data(entity.to_bits() as u128)
        // You can set up an interaction group to allow the collider to interact with the set interaction group
        .solver_groups(InteractionGroups::new(WAY_GROUPS, NONE_GROUPS))
        // After the interaction group is also set up, the collider is allowed to solve the collision under the rules of the group
        .collision_groups(InteractionGroups::new(WAY_GROUPS, NONE_GROUPS))
```

Before going any further into the difference between a solver group and a collision group, we need to understand the rules for constructing an interaction group. We need to provide two parameters for the interaction group's `new()`, the first of which is to specify which group the collider belongs to, the required parameter type is a `u16`, and the second parameter, which sets the collider and which groups of Colliders Will Interact, is also a `u16`.

For the second parameter, it’s easy to understand how settings interact with a single collider, but how do they interact with multiple colliders? This is the beauty of setting the parameter type to `u16`, for example:

```rust
const CREATURE_GROUPS: u16 = 0b0010;
const PLAYER_GROUPS: u16 = 0b0001;
const WALL_GROUPS: u16 = 0b0100;
const WAY_GROUPS: u16 = 0b1000;
const NONE_GROUPS: u16 = 0b0000;
```

The constant is the interaction group variable used in our game, and `0b0011` is represent the `CREATURE` and `PLAYER`, and the number is created using`CREATURE_GROUPS` and `PLAYER_GROUPS` through `&` .

As for the difference between the solution group and the collision group, the solution group is to solve the force situation, and the interaction group will participate in the force solution. The collision group manages collision events, which can be received and processed through `Res<eventqueue>`.

And `user_data`, which is passed into the collider builder when the collider is inserted, can be used to obtain the entity using the following method:

```rust
let entity = Entity::from_bits(user_data as u64);
```

Where does `user_data` come from? From the collision event we get an index that can be accessed via the `Res<ColliderSet>` 's `get()` method user, which is cumbersome and I think the least usable part of `bevy_rapier` so far.

In addition, if you run your game from here, you’ll find that your character, as well as the other dynamic rigid bodies in the screen, will receive a gravitational force , we don’t need this gravity, so we need to change the gravity to zero.

The current version modifies the gravity of the physical engine by adding a startup system like this:

```rust
fn setup(
    mut configuration: ResMut<RapierConfiguration>,
) {
    configuration.gravity = Vector::y() * 0.0;
}
```

Adding this system to `startup_system()` requires only run once when game start-up.

### Multi-platform support

Our game now supports WASM as well as normal desktop platforms, which is no shame since Bevy’s voice wasn’t supported and then wasn’t implemented. After finishing the game to play a little friends, are asking me if there is a mobile version. Bevy’s support plan includes mobile, and while there are few changes to be made to move from desktop to Mobile, before we get to the unsupported mobile, let’s see how we support WASM.

Our game now supports WASM as well as normal desktop platforms, since Bevy’s wasn’t supported audio in WASM version and then we wasn’t implemented this in our game. Bevy’s support platforms includes mobile, and while there are few changes to be made to move from desktop to Mobile, before we talk about this, let’s see how we support WASM.

The bevy rendering backend uses the WGPU, although the WGPU rendering backend supports compiling to WASM, for some reason it’s not loaded on bevy, the bevy WASM project we have been able to refer to is basically based on `bevy_webgl2` this crate.

It’s also convenient to add WASM support, but in addition to adding regular HTML files like other WASM project, you need to make the following changes:

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

Basically this is set up, the rest of the settings are related to HTML, need a little lost knowledge of WASM development. How to use toolchains such as `cargo make` at compile time is also learned from the knowledge developed by WAMS. As for how to deploy to github’s page service, which I don’t know at all, this part of our game was deployed by my partner,`@rgripper` .

For mobile support, Android, for example, if not touch ah, buttons and the like, the official example is actually given, on the basis of the desktop is very convenient to migrate. In addition to the basic android development environment (see [`cargo mobile`](https://github.com/BrainiumLLC/cargo-mobile)’s READEME for details in this section) , just make the following changes to support mobile, even if WGPU support for WASM is later fixed, only the following modifications should be required to support WASM:

```rust
#[bevy_main]
fn main() {
    App::build()
        .insert_resource(Msaa { samples: 2 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
```

### Last part

Many thanks to Rapier author [@Sébastien Crozet](https://github.com/sebcrozet), I’m using the physics engine for the first time, and there are a lot of things I don’t understand that I’ve heard back from people in the discord group.

I would also like to thank my partner [@rgripper](https://github.com/rgripper). Without our cooperation, this project would not have been possible.

I enjoyed developing with my partner and we learned a lot together on this project. If you want to learn about Bevy through practice, then join us and we welcome you to work with us on our next game.

