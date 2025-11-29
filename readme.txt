For the current project, I will only consider a few request options,
which include:
    1. Enter order request (Buy/sell)
    2. Cancel order request
    3. Update order request
I have also included other request types in the struct, but the main focus is to handle the
trading options mentioned above.

In addition, this project has made a few assumptions:
    1. Assume all stock is in the range of $0.01 to $2500.00, outside the range will fail to run.
    2. Ignore field "time in force", all orders will be processed if they appear in order book.

For more details, please refer to the Nasdaq OUCH protocol.
https://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf