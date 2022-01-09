This code generates a random drawing 
similar to this

![img](assets/gen-art.png)

You can change the number of particules in the code by changing the value of the variable `NUM_PARTICLES`
The `SEEd` is used to seed the perlin noise function. Using the same seed, you'll get the same result every time.
Changing the seed will change the result.

## RUN THE CODE
To run the code, you need to have [Rust installed](https://www.rust-lang.org/).
Clone the repo, and run 

```
cargo run --release
```

 This can take a few minutes the first time, but after the first run, it should take less than 2 secondes to rebuilds the code.
 