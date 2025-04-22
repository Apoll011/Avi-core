on_intent
====================

The `on_intent` syntax provides a way to define handlers for specific user intents in your AviScript skills.

## Syntax

```
on_intent "intent_name" {
    // Code to execute when this intent is detected
}
```

## Parameters

- `intent_name`: A string matching the name of an intent defined in your skill's intents directory

## Scope Variables

When an intent handler is triggered, the following constants are available in the scope:

- `name`: The name of the matched intent
- `intent`: The `Intent` object containing extracted slots and information

## Example

```
on_intent "get_weather" {
    let location = intent.optional("location", "current");
    let date = intent.optional("date", "today");
    
    speak.say("weather_report", #{
        "location": location,
        "date": date
    });
    
    // Fetch and display weather data
    let weather_data = http.get("/api/weather", #{ 
        "location": location, 
        "date": date 
    });
    
    // Process weather_data and respond to user
}
```

## Intent Object Methods

The `intent` object provides several methods to access slot data:

- `get(slot_name)`: Get a slot value or null if not present
- `get_raw(slot_name)`: Get the raw slot value
- `require(slot_name)`: Get a slot value or throw an error if not present
- `optional(slot_name, default_value)`: Get a slot value or return default if not present
- `exists(slot_name)`: Check if a slot exists
- `equal(slot_name, value)`: Check if a slot equals a specific value
- `in_list(slot_name, list)`: Check if a slot value is in a list
- `in_dict(slot_name, dict)`: Check if a slot is a key in a dictionary
- `obj()`: Get the full intent object as a map
- `count()`: Get the number of slots
- `all()`: Get all slots as a map