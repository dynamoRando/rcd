# Notes
- Tests are broken out into files to allow the logging system to be init'd. Combining tests into the same file will cause the 
logging system to fail because it's already been init'd.