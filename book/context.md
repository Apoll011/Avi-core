Context Module
=============

The `context` module provides functions for storing and retrieving state information across interactions in your AviScript skills.

## Context Management Functions

### `context.save(name, value)`
Saves a value to the context with the specified name.

Parameters:
- `name`: Context key name
- `value`: Value to save

```
context.save("last_search", "weather in Paris");
context.save("user_preferences", #{ "theme": "dark", "voice": "female_1" });
```

### `context.load(name)`
Loads a value from the context by name.

Parameter:
- `name`: Context key name

Returns:
- `Dynamic`: The stored value or `()` (null) if not found

```
let last_search = context.load("last_search");
if last_search != () {
    speak.text("Your last search was: " + last_search);
}
```

### `context.clear(name)`
Removes a value from the context.

Parameter:
- `name`: Context key name

```
context.clear("temporary_data");
```

## Context Persistence

Context data is maintained across interactions within a session. Some implementations may persist certain context data between sessions, but this should not be assumed unless specifically documented.

## Example Usage

```
on_start {
    // Load user preferences from previous sessions
    let visits = context.load("visit_count") or 0;
    visits += 1;
    context.save("visit_count", visits);
    
    if visits == 1 {
        speak.say("first_time_greeting", #{});
        // Save default preferences
        context.save("preferences", #{ "theme": "light", "notifications": true });
    } else {
        speak.say("return_greeting", #{ "count": visits });
        // Load existing preferences
        let prefs = context.load("preferences");
        if prefs != () {
            // Apply preferences
            if prefs["theme"] == "dark" {
                // Apply dark theme
            }
        }
    }
}

on_intent "update_preference" {
    let pref_name = intent.require("preference_name");
    let pref_value = intent.require("preference_value");
    
    // Load current preferences
    let prefs = context.load("preferences") or #{};
    
    // Update specific preference
    prefs[pref_name] = pref_value;
    
    // Save updated preferences
    context.save("preferences", prefs);
    
    speak.say("preference_updated", #{ "name": pref_name, "value": pref_value });
}

on_intent "clear_history" {
    // Clear sensitive context data
    context.clear("search_history");
    context.clear("last_search");
    
    // Keep preferences
    speak.say("history_cleared", #{});
}
```

## Context Namespacing

It's a good practice to namespace your context keys to avoid conflicts with other skills:

```
// Instead of generic keys
context.save("user", user_data);

// Use namespaced keys
context.save("myskill.user", user_data);
```

## Best Practices

1. Use context to maintain state between interactions
2. Clear temporary or sensitive data when no longer needed
3. Use namespace prefixes for context keys to avoid conflicts
4. Always provide fallback values when loading context
5. Keep context data size reasonable
6. Don't store sensitive information like passwords or tokens in context
7. Use structured data (maps) to group related context values
8. Consider context persistence when designing your skill