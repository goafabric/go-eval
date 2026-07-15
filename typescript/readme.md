# init
echo -- npm init
npm init -y
npm install --save-dev typescript@7
npx tsc --version
      
# compile
echo -- compiling and executing to JAVASCRIPT
echo 'const myName: string = "World"; console.log(`Hello ${myName}!`);' > hello.ts
npx tsc hello.ts
node hello.js

echo -- content of hello.js
cat hello.js

echo -- check tsc binary
file node_modules/@typescript/typescript-darwin-arm64/lib/tsc