import { useState } from "react";
import "./App.css";

function App() {
  return (
    <>
    <h1 className="text-7xl text-blue-600 border">Hello world</h1>
    <Button1 />
    </>
  );
}
function Button1() {
  return(<>
    <button  onClick={onclick} className="rounded-md border text-black px-4 py-2 hover:bg-red-500 active:brightness-90">Hello you piece of shit</button>
  </>);
}
function onclick() {
  alert("You were a bad boy");
}
export default App;
