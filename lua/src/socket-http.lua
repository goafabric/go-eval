-- sudo luarocks install luasocket

local http = require("socket.http")
local ltn12 = require("ltn12")


local url = "http://localhost:50900/callees/sayMyName?name=Heisenberg"
local response = {}

local res, status_code, headers, status = http.request{
    url = url,
    sink = ltn12.sink.table(response)
}

if status_code == 200 then
    print(table.concat(response))
else
    print("HTTP Request failed!", status_code, status)
end
