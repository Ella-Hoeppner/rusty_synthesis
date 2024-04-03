Pure rust DSP and midi handling. Intended mainly for personal use and experimentation.

# to-do
* better low-pass filters
* make listener tree into a dag
  * create a wrapper that stores a signal in an `Arc`, and when sampled stores the current time and result, and if called again with the same time just returns the result
* create templates
  * a wrapper that stores a closure of 0 args that returns a `Signal`
    * should implement would allow for internal randomization of params
* voice allocation
