CoLLector - crawls pages relevant to a given topic and extracts and returns structured node/edge data

NOTE: parrallelized to scrape as many google result links simultaneously as system resources allow

uses `googler` [1] and `gptextract` (`simparse`, `callgpt`) [2]

[1] https://github.com/hversu/googler
[2] https://github.com/hversu/gptextract

## config

update `my_secret.rs` with your API keys
- serpAPI is free (instruction in `my_secret.rs`)
- OpenAI API key required

## usage

`cargo run <topic> <entities>`

## example

`cargo run hversu country,language,culture,era,usage,english_translation`

## returns

(see `data/results.json`)
