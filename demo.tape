Output demo.gif

Set FontSize 24
Set Width 1200
Set Height 800
Set Shell "bash"
Set Theme "Catppuccin Mocha"

# Show help
Type "skatos --help"
Sleep 1s
Enter
Sleep 3s

# Clear screen
Type "clear"
Sleep 500ms
Enter
Sleep 1s

# Set some example environment variables
Type "# Let's set some environment variables with skate 🛹"
Sleep 500ms
Enter
Type "skatos set DATABASE_URL 'postgres://localhost/myapp'"
Sleep 1s
Enter
Sleep 1s

Type "skatos set API_KEY 'sk-1234567890abcdef'"
Sleep 1s
Enter
Sleep 1s

Type "skatos set REDIS_URL 'redis://localhost:6379'"
Sleep 1s
Enter
Sleep 2s

# List all variables
Type "# Let's see what we have stored"
Sleep 500ms
Enter
Type "skatos list"
Sleep 1s
Enter
Sleep 2s

# Generate environment file
Type "# Generate a .env file from our variables"
Sleep 500ms
Enter
Type "skatos env"
Sleep 1s
Enter
Sleep 1s

# Show the generated file
Type "# Let's see what was generated"
Sleep 500ms
Enter
Type "cat .env"
Sleep 1s
Enter
Sleep 3s

# Preview with filter
Type "# Preview with a filter"
Sleep 500ms
Enter
Type "skatos preview --filter 'API'"
Sleep 1s
Enter
Sleep 3s