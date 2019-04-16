import * as wasm from "tetrust";
import { memory } from "tetrust/tetrust_bg";

const CANVAS = document.getElementById("tetrust-canvas");
const GL = CANVAS.getContext("webgl");

if (GL === null) {
  const message = "Unable to initialize WebGL. Your browser or machine may not support it.";
  alert(message);
  throw new Error(message);
}

if (!GL.getExtension('OES_texture_float')) {
  const message = "no floating point texture support";
  alert(message);
  throw new Error(message);
}

// resize canvas to match window
let CANVAS_WIDTH = NaN;
let CANVAS_HEIGHT = NaN;
window.onresize = resize;
resize();

// setup input callbacks
CANVAS.onmousemove = (event) => {
  wasm.on_mouse_move(performance.now(), event.x, event.y);
};

CANVAS.onmousedown = (event) => {
  wasm.on_mouse_down(performance.now(), event.x, event.y);
};

CANVAS.onmouseup = (event) => {
  wasm.on_mouse_up(performance.now(), event.x, event.y);
};

{
  const body = document.getElementsByTagName("BODY")[0];

  body.onkeydown = (event) => {
    if (!event.repeat) {
      wasm.on_key_down(performance.now(), event.keyCode, event.key);
    }
  };

  body.onkeyup = (event) => {
    wasm.on_key_up(performance.now(), event.keyCode, event.key);
  };
}

// TODO: gamepad support
window.addEventListener("gamepadconnected", function(e) {
  console.log(
    "Gamepad connected at index %d: %s. %d buttons, %d axes.",
    e.gamepad.index,
    e.gamepad.id,
    e.gamepad.buttons.length,
    e.gamepad.axes.length
  );
});

// init shader
const SHADER_PROGRAM = GL.createProgram();

GL.attachShader(
  SHADER_PROGRAM,
  loadShader(
    GL,
    GL.VERTEX_SHADER,
    `
      attribute vec4 a_vertex_position;
      attribute vec2 a_tile_map_coord;

      uniform mat4 u_clip_matrix;

      varying highp vec2 vTileMapCoord;

      void main() {
        gl_Position = u_clip_matrix * a_vertex_position;
        vTileMapCoord = a_tile_map_coord;
      }
    `
  )
);

GL.attachShader(
  SHADER_PROGRAM,
  loadShader(
    GL,
    GL.FRAGMENT_SHADER,
    `
      precision highp float;
      varying vec2 vTileMapCoord;

      uniform sampler2D u_tiles;
      uniform sampler2D u_tile_map;
      uniform sampler2D u_src_to_origin;
      uniform sampler2D u_origin_to_dest;

      uniform mat2 u_scale;

      void main() {
        gl_FragColor = texture2D(
          u_tiles,
          u_scale * (vTileMapCoord + texture2D(u_src_to_origin, vTileMapCoord).xy) +
            texture2D(u_origin_to_dest, vec2(texture2D(u_tile_map, vTileMapCoord).a)).xy
        );
      }
    `
  )
);

GL.linkProgram(SHADER_PROGRAM);

if (!GL.getProgramParameter(SHADER_PROGRAM, GL.LINK_STATUS)) {
  const message = "Unable to initialize the shader program: " + GL.getProgramInfoLog(SHADER_PROGRAM);
  alert(message);
  throw new Error(message);
}

{ // init texture coordinate array buffer and bind to attribute
  GL.bindBuffer(GL.ARRAY_BUFFER, GL.createBuffer());

  GL.bufferData(
    GL.ARRAY_BUFFER,
    new Float32Array([
      0,  1,
      1,  1,
      1,  0,
      0,  0,
    ]),
    GL.STATIC_DRAW
  );

  const aTileMapCoord = GL.getAttribLocation(SHADER_PROGRAM, "a_tile_map_coord");

  GL.vertexAttribPointer(
    aTileMapCoord,
    2,        // numComponents
    GL.FLOAT, // type
    false,    // normalize
    0,        // stride
    0         // offset
  );

  GL.enableVertexAttribArray(aTileMapCoord);
}

{ // init element array buffer
  GL.bindBuffer(GL.ELEMENT_ARRAY_BUFFER, GL.createBuffer());

  GL.bufferData(
    GL.ELEMENT_ARRAY_BUFFER,
    new Uint16Array([
      0, 1, 2, 3,
    ]),
    GL.STATIC_DRAW
  );
}

const TILES_TEX = GL.createTexture();
{ // init tiles texture

  GL.bindTexture(GL.TEXTURE_2D, TILES_TEX);

  GL.texImage2D(
    GL.TEXTURE_2D,
    0,                // level
    GL.RGBA,          // internalFormat
    1,                // width
    1,                // height
    0,                // border
    GL.RGBA,          // srcFormat
    GL.UNSIGNED_BYTE, // srcType
    new Uint8Array([228, 92, 16, 255])
  );

  setTextureParameters(GL);

  const image = new Image();

  image.onload = () => {
    GL.bindTexture(GL.TEXTURE_2D, TILES_TEX);

    GL.texImage2D(
      GL.TEXTURE_2D,
      0,                // level
      GL.RGBA,          // internalFormat
      GL.RGBA,          // srcFormat
      GL.UNSIGNED_BYTE, // srcType,
      image
    );

    setTextureParameters(GL);

    startGame();
  };

  image.src = "Tiles.png";
}

const TILE_MAP_TEX = GL.createTexture();
const SRC_TO_ORIGIN_TEX = GL.createTexture();

const TILES_WIDTH = 16;
const TILES_HEIGHT = 16;
const TILES_COUNT = TILES_WIDTH * TILES_HEIGHT;

const ORIGIN_TO_DEST_TEX = GL.createTexture();
{ // init origin to destination texture
  const originToDestArray = [];

  for (let i = 0; i < TILES_HEIGHT; i += 1) {
    for (let j = 0; j < TILES_WIDTH; j += 1) {
      originToDestArray.push(j / TILES_WIDTH);  // s offset
      originToDestArray.push(i / TILES_HEIGHT); // t offset
      originToDestArray.push(0);
    }
  }

  GL.bindTexture(GL.TEXTURE_2D, ORIGIN_TO_DEST_TEX);

  GL.texImage2D(
    GL.TEXTURE_2D,
    0,           // level
    GL.RGB,      // internalFormat
    TILES_COUNT, // width
    1,           // height
    0,           // border
    GL.RGB,      // srcFormat
    GL.FLOAT,    // srcType
    new Float32Array(originToDestArray)
  );

  setTextureParameters(GL);
}

const VERTEX_POSITION_BUF = GL.createBuffer();

GL.useProgram(SHADER_PROGRAM);

// vertex shader locations
const A_VERTEX_POSITION = GL.getAttribLocation(SHADER_PROGRAM, "a_vertex_position");
const U_CLIP_MATRIX = GL.getUniformLocation(SHADER_PROGRAM, "u_clip_matrix");

// fragment shader locations
const U_TILES = GL.getUniformLocation(SHADER_PROGRAM, "u_tiles");
const U_TILE_MAP = GL.getUniformLocation(SHADER_PROGRAM, "u_tile_map");
const U_ORIGIN_TO_DEST = GL.getUniformLocation(SHADER_PROGRAM, "u_origin_to_dest");
const U_SRC_TO_ORIGIN = GL.getUniformLocation(SHADER_PROGRAM, "u_src_to_origin");
const U_SCALE = GL.getUniformLocation(SHADER_PROGRAM, "u_scale");

let GRID_WIDTH = NaN;
let GRID_HEIGHT = NaN;
let SCALE = [1, 0, 0, 1];

let t0 = performance.now();

function startGame() {
  (function iterate(t) {
    if (t - t0 > 32) {
      // console.log(t - t0);
    }

    t0 = t;
    const scene = wasm.render(t, CANVAS_WIDTH, CANVAS_HEIGHT);

    if (scene.changed ||
        scene.grid_width !== GRID_WIDTH ||
        scene.grid_height !== GRID_HEIGHT) {
      try {
        GL.useProgram(SHADER_PROGRAM);

        // update stale srcToOrigin texture
        if (scene.grid_width !== GRID_WIDTH ||
            scene.grid_height !== GRID_HEIGHT) {
          GRID_WIDTH = scene.grid_width;
          GRID_HEIGHT = scene.grid_height;

          const srcToOriginArray = [];

          for (let i = 0; i < GRID_HEIGHT; i += 1) {
            for (let j = 0; j < GRID_WIDTH; j += 1) {
              srcToOriginArray.push(-j / GRID_WIDTH);  // s offset
              srcToOriginArray.push(-i / GRID_HEIGHT); // t offset
              srcToOriginArray.push(0);
            }
          }

          SCALE = [GRID_WIDTH / TILES_WIDTH, 0, 0, GRID_HEIGHT / TILES_HEIGHT];
          GL.bindTexture(GL.TEXTURE_2D, SRC_TO_ORIGIN_TEX);

          GL.texImage2D(
            GL.TEXTURE_2D,
            0,           // level
            GL.RGB,      // internalFormat
            GRID_WIDTH,  // width
            GRID_HEIGHT, // height
            0,           // border
            GL.RGB,      // srcFormat
            GL.FLOAT,    // srcType
            new Float32Array(srcToOriginArray)
          );

          setTextureParameters(GL);

          // update vertex position buffer
          GL.bindBuffer(GL.ARRAY_BUFFER, VERTEX_POSITION_BUF);

          GL.bufferData(
            GL.ARRAY_BUFFER,
            new Float32Array([
              0, 0,
              GRID_WIDTH, 0,
              GRID_WIDTH, GRID_HEIGHT,
              0, GRID_HEIGHT
            ]),
            GL.STATIC_DRAW
          );

          GL.vertexAttribPointer(
            A_VERTEX_POSITION,
            2,        // numComponents
            GL.FLOAT, // type
            false,    // normalize
            0,        // stride
            0         // offset
          );

          GL.enableVertexAttribArray(A_VERTEX_POSITION);
        }

        // update tile map texture
        GL.bindTexture(GL.TEXTURE_2D, TILE_MAP_TEX);
        GL.pixelStorei(GL.UNPACK_ALIGNMENT, 1);

        GL.texImage2D(
          GL.TEXTURE_2D,
          0,
          GL.ALPHA,
          GRID_WIDTH,
          GRID_HEIGHT,
          0,
          GL.ALPHA,
          GL.UNSIGNED_BYTE,
          new Uint8Array(memory.buffer, scene.grid, GRID_WIDTH * GRID_HEIGHT)
        );

        setTextureParameters(GL);

        GL.uniformMatrix4fv(
          U_CLIP_MATRIX,
          false,
          new Float32Array(memory.buffer, scene.clip_matrix, 16)
        );

        setTextureUniform(GL, 0, U_TILES, TILES_TEX);
        setTextureUniform(GL, 1, U_TILE_MAP, TILE_MAP_TEX);
        setTextureUniform(GL, 2, U_SRC_TO_ORIGIN, SRC_TO_ORIGIN_TEX);
        setTextureUniform(GL, 3, U_ORIGIN_TO_DEST, ORIGIN_TO_DEST_TEX);
        GL.uniformMatrix2fv(U_SCALE, false, SCALE);

        // draw
        GL.drawElements(
          GL.TRIANGLE_FAN,
          4,                 // vertexCount
          GL.UNSIGNED_SHORT, // type
          0                  // offset
        );
      } catch (e) {
        console.log(e);
      }
    }

    requestAnimationFrame(iterate);

    function setTextureUniform(gl, unit, uniformLocation, texture) {
      gl.activeTexture(gl.TEXTURE0 + unit);
      gl.bindTexture(gl.TEXTURE_2D, texture);
      gl.uniform1i(uniformLocation, unit);
    }
  })(performance.now());
}

function resize() {
  const realToCSSPixels = window.devicePixelRatio;
  const displayWidth = Math.floor(CANVAS.clientWidth * realToCSSPixels);
  const displayHeight = Math.floor(CANVAS.clientHeight * realToCSSPixels);

  if (CANVAS.width !== displayWidth ||
      CANVAS.height !== displayHeight) {
    CANVAS_WIDTH = displayWidth;
    CANVAS_HEIGHT = displayHeight;
    CANVAS.width = displayWidth;
    CANVAS.height = displayHeight;
    GL.viewport(0, 0, displayWidth, displayHeight);
  }
}

function setTextureParameters(gl) {
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
}

function loadShader(gl, type, source) {
  const shader = gl.createShader(type);
  gl.shaderSource(shader, source);
  gl.compileShader(shader);

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    const message = "An error occurred compiling the shaders: " + gl.getShaderInfoLog(shader);
    gl.deleteShader(shader);
    alert(message);
    throw new Error(message);
  }

  return shader;
}
