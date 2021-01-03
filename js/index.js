import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
ReactDOM.render(<App />, document.getElementById('root'));


// import wabt from "wabt";
// import { window_onload, add_to_carousel, show_result_images, reset_carousel, w_ } from "./window.js";
// console.log(window_onload)
// window.onload = window_onload;

// var features = {
//     exceptions: false,
//     mutable_globals: true,
//     sat_float_to_int: false,
//     sign_extension: false,
//     simd: false,
//     threads: false,
//     multi_value: false,
//     tail_call: false,
//     bulk_memory: false,
//     reference_types: false,
//     console: console
// };


// async function main() {
//     let compiler = await import("../pkg/compiler.js");
//     var images_f = {};
//     var image_files_names = {};

//     document.getElementById('file_upload').addEventListener('change', function (e) {
//         var input_files = e.target.files;
//         [].forEach.call(input_files, function read_file(file) {
//             ////
//             if (/\.(jpe?g|png|gif)$/i.test(file.name)) {
//                 var file_name = file.name.split(".")[0];
//                 var reader = new FileReader();
//                 ////
//                 reader.onload = async function () {
//                     var ctx = document.createElement("canvas").getContext("2d");
//                     var img = document.createElement("img");
//                     ////
//                     img.onload = () => {
//                         ctx.drawImage(img, 0, 0);
//                         var width = img.width;
//                         var height = img.height;
//                         var image_data = ctx.getImageData(0, 0, width, height).data;
//                         var image_data_buffer = new Uint8Array(image_data.buffer);
//                         compiler.add_image_bindgen(file_name, width, height, image_data_buffer);
//                         var v = { "w": width, "h": height, "p": image_data_buffer };
//                         images_f[file_name] = v;
//                         image_files_names = compiler.image_names_bindgen();
//                     };
//                     img.src = reader.result;
//                     add_to_carousel("original_images", file_name, img.src);
//                 };

//                 reader.readAsDataURL(file);
//             }
//         });
//     });

//     document.getElementById('file_rename').onclick = function () {
//         var file_list = document.getElementById("file_list");
//         if (file_list.selectedIndex == -1) {
//             return
//         }

//         var old_name = file_list.options[file_list.selectedIndex].text;
//         var new_name = document.getElementById("file_new_name").value;
//         var error = compiler.rename_image_bindgen(old_name, new_name);
//         while (error.length != 0) {
//             window.alert(error);
//             new_name = document.getElementById("file_new_name").value;
//             error = compiler.rename_image_bindgen(old_name, new_name);
//         }
//         file_list.options[file_list.selectedIndex].text = new_name;
//         image_files_names = compiler.image_names_bindgen();
//     };

//     document.getElementById('file_delete').onclick = function () {
//         var file_list = document.getElementById("file_list");
//         if (file_list.selectedIndex == -1) {
//             return
//         }
//         var file_delete = file_list.options[file_list.selectedIndex].text;
//         file_list.remove(file_list.selectedIndex);
//         compiler.delete_image_bindgen(file_delete);
//         image_files_names = compiler.image_names_bindgen();
//     }

//     document.getElementById('run').onclick = async function () {
//         var code_input = document.getElementById("code_input").value;
//         var image_names = compiler.image_names_bindgen();
//         var output_wasm_buffer = compiler.code2wasm(code_input, image_names);

//         async function print_wat(buffer) {
//             var w = await wabt()
//             var module = w.readWasm(buffer, { readDebugNames: true });
//             module.generateNames();
//             module.applyNames();
//             var wat = module.toText({
//                 foldExprs: true,
//                 inlineExport: false
//             });
//             console.log("wat:", wat);
//             document.getElementById("wat_modal_text").innerHTML = wat;
//         }
//         print_wat(output_wasm_buffer);


//         var importObject = {
//             env: {
//                 darken: function (img_id, value) {
//                     compiler.darken(img_id, value);
//                 },
//                 blank_image: function (width, height) {
//                     compiler.blank_image(width, height);
//                 },
//             }
//         };
//         let { _, instance } = await WebAssembly.instantiate(output_wasm_buffer, importObject);//?
//         instance.exports.main();

//         show_result_images(compiler.export_bindgen());
//     }


//     function show_result_images(result_images) {
//         reset_carousel("result_images");
//         console.log("result images:", result_images);
//         for (let [img_name, img_data] of Object.entries(result_images)) {
//             add_to_carousel("result_images", img_name, image_to_src(img_data.pixels, img_data.width, img_data.height));
//         }
//         compiler.reset_bindgen();
//     }

//     function image_to_src(data, width, height) {
//         var canvas = document.createElement("canvas");
//         canvas.width = width;
//         canvas.height = height;
//         var context = canvas.getContext("2d");
//         var imageData = context.createImageData(width, height);
//         imageData.data.set(data);
//         context.putImageData(imageData, 0, 0);
//         return canvas.toDataURL()
//     }
// }

// main();





