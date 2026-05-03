require 'socket'

port = (ENV['PORT'] || '8080').to_i
server = TCPServer.new('0.0.0.0', port)
puts "listening on :#{port}"
$stdout.flush

loop do
  Thread.start(server.accept) do |client|
    request_line = client.gets || ''
    path = request_line.split(' ')[1] || '/'

    body = if path == '/healthz'
             "ok\n"
           else
             "Hello from kemeter Ruby demo\nRuntime: ruby #{RUBY_VERSION}\n"
           end

    client.print "HTTP/1.1 200 OK\r\nContent-Length: #{body.bytesize}\r\nContent-Type: text/plain\r\n\r\n#{body}"
    client.close
  end
end
