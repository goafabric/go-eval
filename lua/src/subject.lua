-- sudo luarocks install lua-resty-jwt
-- sudo luarocks install lua-cjson
local jwt = require "resty.jwt"  -- Import the lua-resty-jwt library
local x_access_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
local jwt_obj = jwt:verify("lua-resty-jwt", x_access_token)

if jwt_obj then
    local sub = jwt_obj.payload.sub
    ngx.say("sub: " .. sub)
else
    ngx.say("Invalid token")
end
