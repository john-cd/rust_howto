import 'package:flutter/material.dart';
import 'bridge_generated.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text('Flutter Rust Bridge Example'),
        ),
        body: Center(
          child: FutureBuilder<String>(
            future: helloFromRust('World'),
            builder: (context, snapshot) {
              if (snapshot.connectionState == ConnectionState.waiting) {
                return CircularProgressIndicator();
              } else if (snapshot.hasError) {
                return Text('Error: ${snapshot.error}');
              } else {
                return Text('Result: ${snapshot.data}');
              }
            },
          ),
        ),
      ),
    );
  }
}
