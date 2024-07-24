## Case study: LED tree

- First animation: just a basic twinkle.
  - Random duration fade between...
  - ... two random brigthnesses with a min and max
- Fade to black (off) or full multiplier
  - Needs some way of doing layers which are animated/have a duration themselves (in this case it'll
    be a multiply)

```rust
// A "twinkle" effect for a single value
let single_twinkle = Twinkle::<u8>::new(min_value, max_value, min_duration_ms, max_duration_ms);

// 16 twinkling LEDs. Just a wrapper that loops through the array and calls `.tick()` on the effect.
let twinkle = Multi::new([single_twinkle; 16]);

// Not required for twinkle as we the programmer decided that `Twinkle` never ends (i.e.
// `single_twinkle.tick()` will always return `Some(u8)`).
// let animation = LoopForever::new(twinkle);

// Wrap twinkle animation in a layer that allows us to add fade in/fade out e.g. in response to a
// button press.
let mut layer = Layer::new(animation);

let fade_duration_ms = 500;

// Default startup effect: fade in. Current time is 0 at start
layer.set_effect(FadeIn::new(0, fade_duration_ms));

// Fade in
while let Some(fade_in) = layer.effect() {
    // Will remove fade in effect if it's complete
    let value = layer.tick(current_time);

    // Send values to outputs; LEDs, cube, numbers, pixels, whatever
}

// Fade in is done, let's now just fade out
layer.set_effect(FadeOut::new(current_time, fade_duration_ms));

while let Some(fade_out) = layer.effect() {
    // Will remove fade in effect if it's complete
    let value = layer.tick(current_time);

    // Send values to outputs; LEDs, cube, numbers, pixels, whatever
}

// We faded an effect in and out, hooray
```

How the layer might work:

<https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1612f1ed338d11bd490a6fdd504c89f5>

```rust
trait Anim {
    type Output;

    fn anim_stuff(&self, t: u32) -> Option<Self::Output>;
}

trait Eff {
    type Output;

    fn eff_stuff(&self, input: Self::Output, t: u32) -> Option<Self::Output>;
}

struct Layer<A, E> {
    anim: A,
    effect: E,
}

impl<A, E> Layer<A, E> where A: Anim, E: Eff<Output = A::Output> {
    fn tick(&mut self, t: u32) -> Option<E::Output> {
        self.anim.anim_stuff(t)
            .and_then(|res| self.effect.eff_stuff(res, t))
    }
}
```

## Case study: my LED cube

Animation flow is something like this

- Start: all off
- Start slices animation,
- Loop slices _n_ times
- Slices naturally fades out
- Start next animation: rain
- Loop rain _n_ times
- Fade rain to black
- Play blender intro anim
- Blend _n_ times
- Fade blender to black
- GOTO 10

Every animation must have a known duration. This unlocks a lot of stuff like being able to start a
fadeout to end when an underlying animation ends.

Yeah great but how do I do cross fades.

`async` API?

## New notes

Effects overlays? How would we do this with the type system...

struct: this animation has this start and this end, and this easing and this duration

```rust
struct Anim<T, E> {
    start: T,
    end: T,
    easing: E,
    duration_ms: u32
}
```

Not sure how this works with e.g. a crossfade; the animations need to keep going along with the fade
multiply

where `T: Animatable` perhaps. How do we support tweening between two states?

````rust
trait Animatable {
    /// Returning `None` means we're outside this animation's duration and have reached the final value.
    fn tick(&mut self, time: u32) -> Option<T>;

    /// Duration of this animation in milliseconds
    fn duration(&self) -> u32;
}```

that can be wrapped in a looper: once, N, or forever

```rust
struct Looper {
    animation: impl Animatable,
    times: Times
}

enum Times {
    Once,
    Count(usize),
    Forever,
}
````

support nested arrays of animations - this would be a "storyboard" but it also impls the same
animate trait

```rust
let mut storyboard = StoryBoard::new();

storyboard.at(offset_ms).add(Looper { animation: Anim { start: 0.0f32, end: 0.0f32, easing: Lerp, duration_ms: 100 }, times: Times::Count(10) });

```

## Old notes

- `no_std`, duh
- Use `embedded_time::Duration` for everything.
  - Could also use core::time::Duraiton as optimised builds don't seem to harm performance, even
  when converting u32 -> u64 -> Duration -> u128 -> u32. It all
  [compiles out](https://godbolt.org/z/reaWK6). But it costs a lot of storage.
    <!-- - `StatelessAnimation` trait for stateless animations
  <!-- - `StatefulAnimation` trait which takes `&mut self`. Do I even need this? Makes back/forward animations much more difficult --> -->
- `Transition` trait
  - Cross tween two values with a given easing function and duration
  - Takes an easing function as an assoc type/type arg - which one is best?
- `Easing` trait
  - Linear, CSS transitions, sine, etc
- `Driver` trait
  - Infinite looping, N loops, run and stick to end value, etc
