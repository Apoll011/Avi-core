Speak Module
=============

The `speak` module provides functions for text-to-speech, voice control, and managing speech output in your AviScript skills.

## Functions

### Basic Speech

#### `speak.say(key, context)`
Speaks a translated message using a translation key and context for placeholders.

```
speak.say("greeting", #{ "name": "Alex" });  // Says the translated greeting with the name Alex
```

#### `speak.text(message)`
Speaks a literal text message without translation.

```
speak.text("Hello, how can I help you today?");
```

#### `speak.translated(key, context)`
Similar to `say()`, but with different processing (implementation details may vary).

```
speak.translated("farewell", #{ "time": "evening" });
```

### Voice Controls

#### `speak.voice(name)`
Changes the voice used for speech output.

```
speak.voice("female_1");
speak.text("This is spoken in a female voice");

speak.voice("male_2");
speak.text("Now I'm using a male voice");
```

#### `speak.repeat()`
Repeats the last spoken message.

```
speak.text("This is important information");
// Later in the skill
speak.repeat();  // Repeats "This is important information"
```

#### `speak.pause(seconds)`
Inserts a pause of specified duration in seconds.

```
speak.text("I'll give you a moment to think about that.");
speak.pause(3);  // Pause for 3 seconds
speak.text("Now, let's continue.");
```

## Example Usage

```
on_intent "welcome_user" {
    let user_name = intent.optional("name", "friend");
    
    // Set voice based on user preference
    let voice_pref = context.load("voice_preference") or "default";
    speak.voice(voice_pref);
    
    // Greet the user
    speak.say("welcome", #{ "name": user_name });
    speak.pause(1);
    
    // Provide instructions
    speak.say("help_prompt", #{});
}
```

## Best Practices

1. Use translation keys with `speak.say()` rather than hardcoded text to support multiple languages
2. Provide context objects with placeholders for dynamic content
3. Use pauses strategically to make conversation feel more natural
4. Change voices only when it adds value to the user experience
5. Keep utterances concise and clear