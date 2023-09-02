# Stable Diffusion WebUI API Requestor (txt2image)
Calling SD WebUI API using HTTP Request to generate image from command line program. Endpoint, model to use, amount of images to generate, txt2image configuration is configurable.

First run of the binary will generate a configuration file, edit the config file using text editor to configure the default endpoint.

# sd-req
```
sd-req 0.1.0
Stable Diffusion WebUI API Requestor

Arguments
   [repeat/norepeat] [prompt] [count] [CONFIGS..]
Example 1
   repeat "rock in a river" 5
Example 2
   repeat "rock in a river" 1 seed 5 negative_prompt "sand" steps 50
Example 3
   norepeat "rock in a river" 1
```

# Model Configuration
Argument is a pair of key value, add model argument with the model name to change the default model.
```
   norepeat "rock in a river" 1 seed -1 steps 20 model "modelMixTestVersion_modelmixtest.safetensors"
```
