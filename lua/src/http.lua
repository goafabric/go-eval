-- does not work outside http environment server, luarocks install lua-resty-http  
local http = require("resty.http")

local function make_request()
    local httpc = http.new()

    -- URL to call
    local url = "http://localhost:50900/callees/sayMyName?name=Heisenberg"

    -- Perform the GET request
    local res, err = httpc:request_uri(url, {
        method = "GET"
    })

    if not res then
        ngx.say("Failed to request: ", err)
        return
    end

    -- Print the response
    ngx.say("Status: ", res.status)
    ngx.say("Response Body: ", res.body)
end

-- Execute the function
make_request()