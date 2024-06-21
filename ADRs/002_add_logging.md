# Add Logging
Since this tool is grown a little bit since starting as a simple cli tool, some logging/tracing would be good, to help debug in the future.

Decisions to be made:
- Logging or tracing? --> Tracing (with tokio tracing crage)
- Store logs on disk, send elsewhere or log to console? --> console output is fine for the moment