# Issue with static files and session on Firefox 76.0.1

## Steps to reproduce

1. Launch test project
2. Open 127.0.0.1:8080
3. Reload the page and look at the console (should print Count: 0)
4. Open network tab in dev tools
5. Right click on image and click "Open in New Tab"
6. Reload page with opened image
7. Look at the console again

## Expected result
Count is incremented by 1

## Actual result
Count is zero