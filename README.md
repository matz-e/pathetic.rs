# Pathetic

An experimental path tracer written to explore Rust with Python bindings.

## Building

Create the development environment:
```console
$ pipenv install --dev
$ pipenv shell
```

Build the Python bindings with:
```console
$ maturin develop
```

## Example Output

The code in `example.py` will give:

![A rendered scene with spheres](example.jpg)
