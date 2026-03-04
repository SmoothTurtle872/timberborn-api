# Timber born API info

## HTTP Levers:

### Set Parameters

#### Setting Color:

Set the lever to any hex code

`http://localhost:8080/api/color/<lever-name>/<hex-code>`

#### Set Lever state:

set the lever to be on or off. (Different URL for each)

- Setting on: `http://localhost:8080/api/switch-on/<lever-name>`

- Setting off: `http://localhost:8080/api/switch-off/<lever-name>`

### Get Parameters

#### Individual Lever:

Get information about a given lever

`http://localhost:8080/api/levers/<lever-name>`

**Returns:**

```json
{
  "name": "<lever-name>",
  "state": <true|false>,
  "springReturn": <true|false>
}
```

#### All levers:

Get a list of all levers

`http://localhost:8080/api/levers`

**Returns**:

```json
[
    {
      "name": "<lever-name>",
      "state": <true|false>,
      "springReturn": <true|false>
    }
]
```
