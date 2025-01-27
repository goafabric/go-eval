local jwt = require "resty.jwt"
local jwt_obj = jwt:verify("lua-resty-jwt", ngx.req.get_headers()["x-access-token"])
local sub = jwt_obj.payload.sub
ngx.log(ngx.ERR, "subject is: ", sub)