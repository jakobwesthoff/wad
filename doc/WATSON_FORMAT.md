# Watson CLI Format Documentation

This document describes the Watson time tracking CLI tool's data formats and query capabilities.

## Overview

Watson is a command-line time tracking tool that stores time tracking data in "frames" (individual work sessions). It provides several commands for querying this data, with some supporting JSON output format for programmatic access.

## Available Query Commands

### 1. `watson status`
**Purpose**: Shows current project status
**JSON Support**: No
**Example Output**:
```
Project myproject started an hour ago (2024.07.14 15:12:18+0200)
```

**Options**:
- `-p, --project`: Only output project name
- `-t, --tags`: Only show tags
- `-e, --elapsed`: Only show time elapsed

### 2. `watson report` ⭐ **JSON Supported**
**Purpose**: Display time spent on each project (aggregated report)
**JSON Support**: Yes (`--json` flag)

**Key Options**:
- `-f, --from DATETIME`: Start date (defaults to 7 days ago)
- `-t, --to DATETIME`: End date (defaults to tomorrow)
- `-y, --year`: Current year
- `-m, --month`: Current month
- `-w, --week`: Current week
- `-d, --day`: Current day
- `-a, --all`: All activities
- `-c, --current`: Include currently running frame
- `-p, --project TEXT`: Filter by project
- `-T, --tag TEXT`: Filter by tag
- `--ignore-project TEXT`: Exclude projects
- `--ignore-tag TEXT`: Exclude tags
- `-j, --json`: JSON output format
- `-s, --csv`: CSV output format

**JSON Structure**:
```json
{
    "projects": [
        {
            "name": "project_name",
            "tags": [],
            "time": 28800.0
        }
    ],
    "time": 28800.0,
    "timespan": {
        "from": "2024-07-07T00:00:00+02:00",
        "to": "2024-07-14T23:59:59.999999+02:00"
    }
}
```

**Notes**:
- Time values are in seconds (float)
- When `--current` is used, time includes running session with decimal precision
- Tags array contains tag objects with name and time if present

### 3. `watson log` ⭐ **JSON Supported**
**Purpose**: Display individual recorded sessions (frames)
**JSON Support**: Yes (`--json` flag)

**Key Options** (same timeframe options as report):
- `-c, --current`: Include currently running frame
- `-r, --reverse`: Reverse order of days
- `-j, --json`: JSON output format
- `-s, --csv`: CSV output format
- Same filtering options as report command

**JSON Structure**:
```json
[
    {
        "id": "a1b2c3d4e5f6789012345678901234ab",
        "project": "myproject",
        "start": "2024-07-14T09:17:59+02:00",
        "stop": "2024-07-14T14:52:32+02:00",
        "tags": ["development", "frontend"]
    },
    {
        "id": "current",
        "project": "myproject",
        "start": "2024-07-14T15:12:18+02:00",
        "stop": "2024-07-14T17:03:32.399976+02:00",
        "tags": ["testing"]
    }
]
```

**Notes**:
- Returns array of frame objects
- Each frame has unique ID (hash) except current session which has id "current"
- Start/stop times in ISO 8601 format with timezone
- Current session stop time updates in real-time with microsecond precision
- Tags array contains tag names as strings

### 4. `watson aggregate` ⭐ **JSON Supported**
**Purpose**: Day-by-day breakdown of time spent on projects
**JSON Support**: Yes (`--json` flag)

**Key Options**:
- `-c, --current`: Include currently running frame
- `-f, --from DATETIME`: Start date
- `-t, --to DATETIME`: End date
- `-p, --project TEXT`: Filter by project
- `-T, --tag TEXT`: Filter by tag
- `-j, --json`: JSON output format
- `-s, --csv`: CSV output format

**Note**: Does not support shortcut timeframe options (--day, --week, etc.)

**JSON Structure**:
```json
[
    {
        "projects": [
            {
                "name": "myproject",
                "tags": ["development"],
                "time": 25200.0
            }
        ],
        "time": 25200.0,
        "timespan": {
            "from": "2024-07-07T00:00:00+02:00",
            "to": "2024-07-07T23:59:59.999999+02:00"
        }
    },
    {
        "projects": [],
        "time": 0.0,
        "timespan": {
            "from": "2024-07-11T00:00:00+02:00",
            "to": "2024-07-11T23:59:59.999999+02:00"
        }
    }
]
```

**Notes**:
- Returns array of daily aggregations
- Each day has its own timespan and projects breakdown
- Days with no activity show empty projects array and time: 0.0
- Useful for creating daily/weekly charts and calendars

### 5. `watson projects`
**Purpose**: List all existing projects
**JSON Support**: No
**Example Output**:
```
myproject
website-redesign
client-work
```

### 6. `watson tags`
**Purpose**: List all existing tags
**JSON Support**: No
**Example Output**: (returns tag names, one per line)

### 7. `watson frames`
**Purpose**: List all frame IDs
**JSON Support**: No
**Example Output**:
```
a1b2c3d
4e5f6g7
8h9i0j1
...
```

## Data Types and Formats

### Time Values
- **Format**: Float (seconds)
- **Example**: `28800.0` = 8 hours, 0 minutes, 0 seconds
- **Precision**: When current session is included, decimal precision shows sub-second timing

### Timestamps
- **Format**: ISO 8601 with timezone
- **Example**: `"2024-07-14T09:17:59+02:00"`
- **Current Session**: Includes microsecond precision: `"2024-07-14T17:03:32.399976+02:00"`

### Frame IDs
- **Format**: Hexadecimal hash string
- **Length**: Variable (typically 32 characters)
- **Example**: `"a1b2c3d4e5f6789012345678901234ab"`
- **Special Case**: Current session uses `"current"` as ID

### Project Names
- **Format**: Free text string
- **Examples**: `"myproject"`, `"website-redesign"`, `"Client Project ABC"`
- **Note**: Can contain spaces and special characters

### Tags
- **Format**: Array of strings in JSON, space-separated in text output
- **Example**: `["development", "frontend", "testing"]`

## Command Combinations for Dashboard Use

### Current Status
```bash
watson status
```

### Today's Summary
```bash
watson report --json --day --current
```

### Today's Sessions
```bash
watson log --json --day --current
```

### Weekly Overview
```bash
watson report --json --week --current
```

### Weekly Daily Breakdown
```bash
watson aggregate --json --from $(date -d '7 days ago' +%Y-%m-%d) --current
```

### Monthly Summary
```bash
watson report --json --month --current
```

### All Projects List
```bash
watson projects
```

### Specific Project Analysis
```bash
watson report --json --project "project_name" --month
watson log --json --project "project_name" --month
```

## Best Practices for Dashboard Integration

1. **Use JSON Output**: Always prefer `--json` flag when available for programmatic parsing
2. **Include Current Session**: Use `--current` flag to get real-time data
3. **Time Calculations**: Convert seconds to human-readable format (hours:minutes:seconds)
4. **Error Handling**: Commands may return empty arrays/objects when no data exists
5. **Timezone Awareness**: All timestamps include timezone information
6. **Rate Limiting**: Watson commands are generally fast, but avoid excessive polling for current status

## Time Conversion Utilities

```bash
# Convert seconds to hours:minutes:seconds
# Example: 28800 seconds = 08:00:00
seconds_to_hms() {
    local seconds=$1
    printf "%02d:%02d:%02d" $((seconds/3600)) $(((seconds%3600)/60)) $((seconds%60))
}
```

## Configuration

Watson configuration can be queried and modified using:
```bash
watson config section.option [value]
watson config --edit  # Opens config file in editor
```

This is useful for customizing date/time formats and other display options.
