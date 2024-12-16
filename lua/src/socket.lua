-- sudo luarocks install luasocket

local http = require("socket.http")
local ltn12 = require("ltn12")

-- URL to call
local url = "http://localhost:50900/callees/sayMyName?name=Heisenberg"

-- Buffer to hold the response
local response = {}

-- Perform the GET request
local res, status_code, headers, status = http.request{
    url = url,
    sink = ltn12.sink.table(response)
}

-- Check the status code
if status_code == 200 then
    -- Print the response
    print("Response:")
    print(table.concat(response))
else
    print("HTTP Request failed!")
    print("Status code:", status_code)
    print("Status:", status)
end
