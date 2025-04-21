Ask Module
=============

The `ask` module provides functions for gathering input from users in your AviScript skills.

## Functions

### `ask.question(key, callback, context, expected)`
Asks a question using a translation key and calls the callback when the user responds.

Parameters:
- `key`: Translation key for the question
- `callback`: Function to call with the user's response
- `context`: Map of placeholder values for the question text
- `expected`: Expected input type or validation criteria

```
ask.question("ask_name", |response| {
    context.save("user_name", response);
    speak.say("greeting", #{ "name": response });
}, #{}, "text");
```

### `ask.on_input(callback, expected)`
Sets up a handler for the next user input.

Parameters:
- `callback`: Function to call with the user's input
- `expected`: Expected input type or validation criteria

```
speak.text("What would you like to do next?");
ask.on_input(|input| {
    if input.contains("weather") {
        // Handle weather intent
    } else if input.contains("news") {
        // Handle news intent
    } else {
        speak.text("I didn't understand that.");
    }
}, "text");
```

### `ask.confirm(callback)`
Asks for a yes/no confirmation and calls the callback with the result.

Parameter:
- `callback`: Function to call with boolean result (true for yes, false for no)

```
speak.text("Are you sure you want to delete your profile?");
ask.confirm(|confirmed| {
    if confirmed {
        // Delete profile
        speak.text("Your profile has been deleted.");
    } else {
        speak.text("Operation cancelled.");
    }
});
```

### `ask.cancel(callback)`
Sets up a handler for if the user cancels the current interaction.

Parameter:
- `callback`: Function to call when user cancels

```
ask.cancel(|| {
    speak.text("No problem, we can do this later.");
    context.save("setup_completed", false);
});
```

### `ask.number_input(prompt, callback)`
Asks specifically for a number input.

Parameters:
- `prompt`: Text prompt for the number input
- `callback`: Function to call with the number response

```
ask.number_input("How many tickets would you like to purchase?", |count| {
    if count > 0 && count <= 10 {
        context.save("ticket_count", count);
        speak.text("Added " + count + " tickets to your cart.");
    } else {
        speak.text("Sorry, you can only purchase between 1 and 10 tickets.");
    }
});
```

## Example Usage

```
on_intent "book_flight" {
    // Ask for destination
    ask.question("ask_destination", |destination| {
        context.save("flight_destination", destination);
        
        // Ask for date
        ask.question("ask_travel_date", |date| {
            context.save("flight_date", date);
            
            // Confirm booking
            speak.say("confirm_booking", #{ 
                "destination": destination,
                "date": date
            });
            
            ask.confirm(|confirmed| {
                if confirmed {
                    // Process booking
                    speak.say("booking_confirmed", #{});
                } else {
                    speak.say("booking_cancelled", #{});
                }
            });
        }, #{}, "date");
    }, #{}, "text");
    
    // Set up cancel handler
    ask.cancel(|| {
        speak.say("booking_process_cancelled", #{});
    });
}
```

## Best Practices

1. Always provide clear prompts for what kind of input you expect
2. Handle unexpected inputs gracefully
3. Use appropriate input types (text, number, confirmation)
4. Always provide a cancel handler for multi-step interactions
5. Save user responses to context for later use