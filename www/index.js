import * as wasm from "wasm-pack-lib"

// initializes pixi instance (see browser console log for pixi message)
const app = wasm.create_pixi_app()

// document.body.appendChild(app.view);
// app.renderer.backgroundColor = 0x20202e;
// let type = "WebGL";
// if(!PIXI.utils.isWebGLSupported()){
//     type = "canvas";
// }
//
// PIXI.utils.sayHello(type);
//
// PIXI.loader
//     .add("testface", "./img/testface.png")
//     .load(setup);
//
// let sprite;
//
// function setup() {
//     sprite = new PIXI.Sprite(
//         PIXI.loader.resources["testface"].texture
//     );
//     //sprite.position.set(256, 128);
//     sprite.anchor.set(0.5, 0.5);
//     const sprite2 = rust.add_sprite("testface");
//     sprite.
//     sprite2.
//     app.stage.addChild(sprite);
//     app.stage.addChild(sprite2);
// }


