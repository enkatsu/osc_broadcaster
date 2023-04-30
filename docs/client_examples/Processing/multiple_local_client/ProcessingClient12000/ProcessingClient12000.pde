import oscP5.*;
import netP5.*;


OscP5 oscP5;
NetAddress myBroadcastLocation;
ArrayList<Circle> circles;

void setup() {
  size(400, 400);
  frameRate(25);
  oscP5 = new OscP5(this, "127.0.0.1", 12000, OscP5.UDP);
  myBroadcastLocation = new NetAddress("127.0.0.1", 32000);
  circles = new ArrayList<Circle>();
}


void draw() {
  background(0);
  for (Circle circle: circles) {
    circle.draw();
  }
  text("'c': connect\n'd': disconnect", 10, 20);
}


void mousePressed() {
  OscMessage myOscMessage = new OscMessage("/mouse/pressed");
  myOscMessage.add(mouseX);
  myOscMessage.add(mouseY);
  oscP5.send(myOscMessage, myBroadcastLocation);
}


void keyPressed() {
  OscMessage m;
  switch(key) {
    case('c'):
      m = new OscMessage("/server/connect", new Object[0]);
      m.add(oscP5.properties().listeningPort());
      oscP5.send(m, myBroadcastLocation);
      break;
    case('d'):
      m = new OscMessage("/server/disconnect", new Object[0]);
      m.add(oscP5.properties().listeningPort());
      oscP5.send(m, myBroadcastLocation);
      break;
  }
}

void oscEvent(OscMessage message) {
  String addr = message.addrPattern();
  String typetag = message.typetag();
  println("### received an osc message with addrpattern " + addr + " and typetag " + typetag);
  message.print();
  int x = message.get(0).intValue();
  int y = message.get(1).intValue();
  circles.add(new Circle(x, y, 50, color(100, 200, 250)));
}

class Circle {
  float x, y, r;
  color c;
  
  Circle(float x, float y, float r, color c) {
    this.x = x;
    this.y = y;
    this.r = r;
    this.c = c;
  }

  void draw() {
    push();
    noStroke();
    fill(c);
    ellipse(x, y, r, r);
    pop();
  }
}
