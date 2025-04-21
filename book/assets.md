Assets Module
=============

The `assets` module provides functions for accessing and managing skill assets like files, images, and audio in your AviScript skills.

## File Functions

### `assets.get(file)`
Gets the full path to an asset file.

Parameter:
- `file`: Relative path to the asset file

Returns:
- `String`: Full path to the asset file

```
let image_path = assets.get("images/logo.png");
```

### `assets.exists(file)`
Checks if an asset file exists.

Parameter:
- `file`: Relative path to the asset file

Returns:
- `bool`: True if the file exists, false otherwise

```
if assets.exists("data/user_preferences.json") {
    // Use the file
} else {
    // Create default preferences
}
```

### `assets.read_text(file)`
Reads a text file and returns its contents as a string.

Parameter:
- `file`: Relative path to the text file

Returns:
- `String`: Contents of the file

```
let instructions = assets.read_text("instructions.txt");
speak.text(instructions);
```

### `assets.read_json(file)`
Reads a JSON file and returns its parsed contents as a map.

Parameter:
- `file`: Relative path to the JSON file

Returns:
- `Map`: Parsed JSON content

```
let config = assets.read_json("config/settings.json");
let timeout = config["timeout"] or 30;
```

## Audio Submodule

The `assets.audio` submodule provides functions for playing and controlling audio files.

### `assets.audio.play(file)`
Plays an audio file.

Parameter:
- `file`: Relative path to the audio file

```
assets.audio.play("sounds/notification.mp3");
```

### `assets.audio.stop()`
Stops the currently playing audio.

```
assets.audio.stop();
```

### `assets.audio.is_playing()`
Checks if audio is currently playing.

Returns:
- `bool`: True if audio is playing, false otherwise

```
if assets.audio.is_playing() {
    assets.audio.stop();
}
```

### `assets.audio.volume(level)`
Sets the audio volume level.

Parameter:
- `level`: Volume level (0-100)

```
assets.audio.volume(75);  // Set volume to 75%
```

### `assets.audio.mute()`
Mutes the audio.

```
assets.audio.mute();
```

### `assets.audio.unmute()`
Unmutes the audio.

```
assets.audio.unmute();
```

## Example Usage

```
on_intent "play_tutorial" {
    // Check if tutorial exists
    if assets.exists("tutorials/beginner.mp3") {
        speak.text("Playing the beginner tutorial now.");
        
        // Set volume and play
        assets.audio.volume(80);
        assets.audio.play("tutorials/beginner.mp3");
        
        // Display accompanying image
        
        // Load supplementary text
        let tips = assets.read_text("tutorials/beginner_tips.txt");
        context.save("current_tips", tips);
    } else {
        speak.text("Sorry, the tutorial isn't available.");
    }
}
```

## Best Practices

1. Always check if files exist before attempting to use them
2. Use appropriate file formats for different asset types
3. Keep audio files short and high-quality
4. Organize assets in a logical directory structure
5. Use volume controls responsibly to prevent jarring audio experiences