import * as three from "jsr:@3d/three";
export function szene() {
  return (<>
  const scene = three.Scene;
  const camera = new three.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1 ,1000);
  const render = new three.WebGLRenderer();
  render.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(render.domElement);


  const geometry = new three.BoxGeometry(1,1,1);
  const material =  new three.MeshBasicMaterial({color:0x00ff00});
  const cube = new three.Mesh(geometry,material);
  render.render(scene,camera);
  </>);
}
