Http Module
=============


The `http` module provides functions for making HTTP requests to external services from your AviScript skills.

## HTTP Request Functions

### `http.call(route, method, params)`
Makes an HTTP request with the specified method and parameters.

Parameters:
- `route`: URL or endpoint path
- `method`: HTTP method (GET, POST, PUT, DELETE, etc.)
- `params`: Map of parameters or body data

Returns:
- `Dynamic`: Response data, typically parsed from JSON

```
let response = http.call("https://api.example.com/data", "GET", #{ "id": 123 });
```

### `http.get(route, params)`
Makes an HTTP GET request.

Parameters:
- `route`: URL or endpoint path
- `params`: Map of query parameters

Returns:
- `Dynamic`: Response data, typically parsed from JSON

```
let weather_data = http.get("/api/weather", #{ 
    "location": "London", 
    "units": "metric" 
});

speak.text("The temperature is " + weather_data["temperature"] + " degrees");
```

### `http.post(route, body)`
Makes an HTTP POST request.

Parameters:
- `route`: URL or endpoint path
- `body`: Map of data to send in the request body

Returns:
- `Dynamic`: Response data, typically parsed from JSON

```
let result = http.post("/api/users", #{ 
    "name": "Alice Smith", 
    "email": "alice@example.com" 
});

if result["success"] {
    speak.text("User created successfully");
} else {
    speak.text("Failed to create user: " + result["error"]);
}
```

### `http.status()`
Gets the status code of the last HTTP request.

Returns:
- `int`: HTTP status code

```
let data = http.get("/api/products", #{ "category": "electronics" });

if http.status() == 200 {
    // Handle successful response
    speak.text("Found " + data["count"] + " products");
} else if http.status() == 404 {
    speak.text("No products found in that category");
} else {
    speak.text("Error fetching products: " + http.status());
}
```

## Error Handling

HTTP requests can fail for various reasons. Always check the status code or handle errors appropriately:

```
let response = http.get("/api/data", #{ "id": user_id });

if http.status() >= 400 {
    speak.say("api_error", #{ "status": http.status() });
    return;
}

// Process successful response
```

## Example Usage

```
on_intent "get_weather_forecast" {
    let location = intent.require("location");
    let days = intent.optional("days", 1);
    
    speak.say("fetching_weather", #{ "location": location });
    
    // Make API request
    let weather = http.get("https://weather-api.example.com/forecast", #{ 
        "location": location, 
        "days": days 
    });
    
    // Check for errors
    if http.status() != 200 {
        speak.say("weather_error", #{ "status": http.status() });
        return;
    }
    
    // Process and present the data
    let forecast = weather["forecast"];
    let summary = weather["summary"];
    
    speak.say("weather_report", #{ 
        "location": location,
        "summary": summary,
        "temperature": forecast["temperature"],
        "conditions": forecast["conditions"]
    });
    
    // Save the request for later reference
    context.save("last_weather_location", location);
}
```

## Best Practices

1. Always handle HTTP errors gracefully
2. Use HTTPS URLs for security
3. Cache responses when appropriate to reduce API calls
4. Implement rate limiting for frequent requests
5. Keep authentication credentials secure
6. Structure API endpoints consistently
7. Use appropriate HTTP methods for different operations
8. Include timeout handling for slow responses
9. Log