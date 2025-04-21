on_start/on_end
=============

The `on_start` and `on_end` syntax provides lifecycle hooks for your AviScript skills.

## on_start

The `on_start` handler is executed when your skill is initialized, before any intents are processed.

### Syntax

```
on_start {
    // Code to execute when skill starts
}
```

### Example

```
on_start {
    // Initialize skill state
    let welcome_shown = context.load("welcome_shown");
    
    if !welcome_shown {
        speak.say("welcome_message", #{});
        context.save("welcome_shown", true);
    }
    
    // Load user preferences
    let user_settings = assets.read_json("user_settings.json");
    context.save("settings", user_settings);
}
```

## on_end

The `on_end` handler is executed when your skill is stopping or being unloaded.

### Syntax

```
on_end {
    // Code to execute when skill ends
}
```

### Example

```
on_end {
    // Clean up resources
    assets.audio.stop();
    
    // Save user state
    let user_data = context.load("user_data");
    assets.write_json("user_data.json", user_data);
    
    // Say goodbye
    speak.say("goodbye_message", #{});
}
```

## Usage Notes

- `on_start` runs exactly once when the skill is loaded
- `on_end` runs when the skill is explicitly stopped or unloaded
- Use these handlers to initialize and clean up resources, load and save persistent data, and manage user experience