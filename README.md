# Crop Yield and Harvest Tracking System

### Introduction

This is a system designed to manage and track crop yields, growth stages, weather conditions and applications of fertilizers and pestcides. Its a Rust Smart Contract intended to run on the Internet Computer Protocol blockchain layer. 

### Features

#### 1. Crop Management

  * `Create new crop entries with details like crop type, variety, field location, planting date, and expected yield.`
  * `Update crop details such as actual yield, infection monitoring, pest and disease details, soil quality, and pH level.`
  * `Delete crop entries.`

#### 2. Growth Stage Tracking

  * `Track the growth stage of crops (e.g., Planting, Germination, Vegetative, Flowering, Fruiting, Ripening, Harvesting).`
  * `Update and query the current growth stage of a crop.`

#### 3. Fertilizer and Pesticide Application Tracking

  * `Log applications of fertilizers and pesticides for each crop.`
  * `Retrieve application details for a specific crop.`

#### 4. Weather Conditions

  * `Track weather conditions (e.g., rainfall, temperature) for each crop.`

#### 5. Yield Prediction

  * `Predict crop yield based on factors like weather conditions, soil quality, fertilizer usage, and pesticide usage.`

#### 6. Data Storage

  * `Store crop data in a thread-safe, mutable `HashMap` using `RefCell` for internal mutability.`


## Prerequisites

Before running the project, ensure you have the following installed:

* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```


## Running the project locally

If you want to test the project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```