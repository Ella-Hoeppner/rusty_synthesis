Pure rust DSP and midi handling. Intended mainly for personal use and experimentation.

# to-do
* figure out why `derive_signal_ops!(Cached<S: Signal>)` causes problems
  * guessing it has to do with the `Arc<Mutex<_>>`?
* add a version of `DetunedSum::even` and `::random` that can dynamically change the detune amount
  * I think the impl signature will be something like:
    ```
    impl<F: Signal, D: Signal, S: Signal + Clone> DetunedSum<FreqMod<Pow<Const, D>, F>, S>
    ```
    because it needs to have a type `D` for the detune signal, which gets exponentiated to put it in frequency-space, and that needs to frequncy-modulate `F`
* make midi notes just store an `Arc<Mutex<bool>>`, and have the midi controller modify that bool
* save and load midi state
  * at least the state of the modwheel. Will also eventually wanna add the knobs on my keystep once it's fixed
* fix ADSR overlaps
* create templates
  * a wrapper that stores a closure of 0 args that returns a `Signal`
    * would allow for internal randomization of params
* effects to implement
  * Fay recommendations <3
    * pulse-width modulatio
    * butterworth filter
    * freeverbe
    * dattorro reverb
    * karplus strong
* consider options for adding a GUI
  * would be nice to be able to bind values to different virtual sliders and stuff
    * could also have a way to easily map any slider in the GUI to the mod wheel/knobs on the midi controller
  * could just do wgpu, might be kinda heavy-duty tho
    * doubt I could find a GUI library that would look as nice as what I could make tho...
    * what about makepad?
