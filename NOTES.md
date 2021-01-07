- `no_std`, duh
- Use `embedded_time::Duration` for everything.
  - Could also use core::time::Duraiton as optimised builds don't seem to harm performance, even when converting u32 -> u64 -> Duration -> u128 -> u32. It all [compiles out](https://godbolt.org/z/reaWK6). But it costs a lot of storage.
    <!-- - `StatelessAnimation` trait for stateless animations
  <!-- - `StatefulAnimation` trait which takes `&mut self`. Do I even need this? Makes back/forward animations much more difficult --> -->
- `Transition` trait
  - Cross tween two values with a given easing function and duration
  - Takes an easing function as an assoc type/type arg - which one is best?
- `Easing` trait
  - Linear, CSS transitions, sine, etc
- `Driver` trait
  - Infinite looping, N loops, run and stick to end value, etc
