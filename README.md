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

# sd-req.json
```
{
   "url" : "http://127.0.0.1:7860",
   "output_path" : "output",
   "model" : "",
   "negative_prompt" : "",
   "steps" : 20,
   "width" : 512,
   "height" : 512,
   "sampler_index" : "Euler",
   "cfg_scale" : 7,
   "tiling" : false,
   "n_iter" : 1,
   "batch_size" : 1,
   "restore_faces" : false,
   "denoising_strength" : 0,
   "firstphase_width" : 0,
   "firstphase_height" : 0,
   "seed" : -1,
   "subseed" : -1,
   "subseed_strength" : 0,
   "seed_resize_from_h" : -1,
   "seed_resize_from_w" : -1,
   "eta" : 0,
   "s_churn" : 0,
   "s_tmax" : 0,
   "s_tmin" : 0,
   "s_noise" : 1
}
```
Generated with default value when not exist, edit to change default configuration such as Endpoint URL.

# Requirement
Stable Diffusion WebUI need to enable to publish API by adding `COMMANDLINE_ARGS` with `--api` read [here](https://github.com/AUTOMATIC1111/stable-diffusion-webui/wiki/Command-Line-Arguments-and-Settings)
