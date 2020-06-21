# envers

## Description

This library is a new data format for serde allowing you to only Serialize to env vars.

## Why not use envy?

At the time, envy only allows serialization.

## First step

First step is to enable serialization, as envy does not do it at all for now.
The base of this is done from the [serde data-format example repository](https://github.com/serde-rs/example-format).
