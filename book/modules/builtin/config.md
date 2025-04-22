Config Module
=============

The `config` module provides functions for accessing and managing skill configuration settings in your AviScript skills.

## Config Management Functions

### `config.get(name)`
Gets a configuration value by name.

Parameter:
- `name`: Configuration key name

Returns:
- `Dynamic`: The configuration value

```
let timeout = config.get("request_timeout");
```

### `config.set(name, value)`
Sets a configuration value.

Parameters:
- `name`: Configuration key name
- `value`: Value to set

```
config.set("user_preferences.theme", "dark");
```

### `config.has(name)`
Checks if a configuration key exists.

Parameter:
- `name`: Configuration key name

Returns:
- `bool`: True if the configuration exists, false otherwise

```
if config.has("api_key") {
    // Use the API key
} else {
    speak.text("Please configure your API key first.");
}
```

### `config.type_of(name)`
Gets the data type of a configuration value.

Parameter:
- `name`: Configuration key name

Returns:
- `String`: Type name ("string", "int", "float", "bool", "list", "map")

```
let value_type = config.type_of("max_results");
if value_type == "int" {
    // Handle integer config
} else if value_type == "string" {
    // Handle string config
}
```

### `config.constant(name)`
Gets a constant value by name.

Parameter:
- `name`: Constant name

Returns:
- `String`: The constant value

```
let api_endpoint = config.constant("API_ENDPOINT");
```

## Configuration File Structure

Configurations are typically stored in `skill.config` in your skill directory, structured as a JSON file with a `configs` and `constants` section:

```json
{
  "configs": {
    "request_timeout": {
      "value": 30,
      "default": 30,
      "type": "int",
      "label": "Request Timeout",
      "description": "Timeout in seconds for API requests"
    },
    "max_results": {
      "value": 5,
      "default": 5,
      "type": "int",
      "label": "Maximum Results",
      "description": "Maximum number of results to display"
    }
  },
  "constants": {
    "API_ENDPOINT": "https://api.example.com/v1",
    "VERSION": "1.0.0",
    "DEBUG_MODE": false
  }
}
```

## Example Usage

```
on_start {
    // Check and validate configurations
    if !config.has("api_key") {
        speak.text("API key not configured. Some features may not work.");
    }
    
    // Get and use configs with defaults
    let timeout = 30;
    if config.has("request_timeout") {
        timeout = config.get("request_timeout");
    }
    
    // Use constants
    let version = config.constant("VERSION");
    speak.text("Running skill version " + version);
    
    // Save the timeout to context for later use
    context.save("current_timeout", timeout);
}

on_intent "change_settings" {
    let setting_type = intent.get("setting_type");
    let new_value = intent.get("setting_value");
    
    if setting_type == "timeout" {
        if config.type_of("request_timeout") == "int" {
            // Convert string to number
            let timeout_value = parse_int(new_value);
            config.set("request_timeout", timeout_value);
            speak.say("setting_updated", #{ "setting": "timeout", "value": new_value });
        }
    } else if setting_type == "theme" {
        config.set("theme", new_value);
        speak.say("setting_updated", #{ "setting": "theme", "value": new_value });
    }
}
```

## Best Practices

1. Always provide sensible default values
2. Check if configurations exist before using them
3. Validate configuration types and values
4. Use constants for values that shouldn't change at runtime
5. Organize related configurations using dot notation (e.g., "audio.volume", "audio.quality")
6. Include descriptive labels and descriptions in your config file