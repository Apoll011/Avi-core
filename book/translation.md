Translation Module
=============

The `translation` module provides functions for accessing and managing translations in your AviScript skills, enabling multi-language support.

## Translation Functions

### `translation.get(key, context)`
Gets a translated string by key, with placeholder replacements.

Parameters:
- `key`: Translation key
- `context`: Map of placeholder values

Returns:
- `String`: Translated text with placeholders replaced

```
let greeting = translation.get("welcome_user", #{ "name": "Alex" });
// Might return: "Welcome, Alex!" or "Bienvenido, Alex!" depending on language
```

### `translation.get_raw(key)`
Gets a translated string by key without any formatting.

Parameter:
- `key`: Translation key

Returns:
- `String`: Raw translated text

```
let raw_text = translation.get_raw("welcome_message");
// Might return: "Welcome to our service, {user}!"
```

### `translation.exists(key)`
Checks if a translation key exists.

Parameter:
- `key`: Translation key

Returns:
- `bool`: True if the translation exists, false otherwise

```
if translation.exists("advanced_feature_description") {
    speak.translated("advanced_feature_description", #{});
} else {
    speak.text("This feature helps you automate tasks.");
}
```

### `translation.get_or(key, fallback)`
Gets a translation or falls back to a default if not found.

Parameters:
- `key`: Translation key
- `fallback`: Fallback text if key not found

Returns:
- `String`: Translated text or fallback

```
let message = translation.get_or("rare_error_code_5", "An unexpected error occurred.");
```

### `translation.format_with_placeholders(base, context)`
Formats a string with placeholders from a context map.

Parameters:
- `base`: Text with placeholders in format {placeholder}
- `context`: Map of placeholder values

Returns:
- `String`: Formatted text

```
let template = "Hello, {name}! You have {count} new messages.";
let formatted = translation.format_with_placeholders(template, #{ 
    "name": "Taylor", 
    "count": 5 
});
// Returns: "Hello, Taylor! You have 5 new messages."
```

## Translation System

Translations are typically stored in language-specific files in the `responses` directory of your skill:

```
responses/
├── en/
│   └── translations.json
└── es/
    └── translations.json
```

Example `translations.json` for English:
```json
{
  "welcome_user": "Welcome, {name}!",
  "goodbye": "Goodbye, see you soon!",
  "help_prompt": "What can I help you with today?",
  "error_not_found": "Sorry, I couldn't find that information."
}
```

## Example Usage

```
on_start {
    // Get current language
    let current_lang = context.load("user_language") or "en";
    
    // Welcome message with name
    let user_name = context.load("user_name") or "friend";
    let welcome = translation.get("welcome_user", #{ "name": user_name });
    speak.text(welcome);
}

on_intent "help_request" {
    let topic = intent.optional("topic", "");
    
    if topic != "" && translation.exists("help_" + topic) {
        // Topic-specific help
        speak.translated("help_" + topic, #{});
    } else {
        // General help
        speak.translated("help_general", #{});
        
        // List available topics
        let help_topics = ["accounts", "payments", "settings"];
        let topics_text = translation.get("available_help_topics", #{ 
            "topics": help_topics.join(", ") 
        });
        speak.text(topics_text);
    }
}
```

## Best Practices

1. Use translation keys for all user-facing text
2. Organize translations by feature or screen
3. Use descriptive key names that indicate the purpose of the text
4. Keep placeholders consistent across languages
5. Provide fallbacks for missing translations
6. Use raw translations when you need to perform custom formatting
7. Consider cultural differences when designing translations