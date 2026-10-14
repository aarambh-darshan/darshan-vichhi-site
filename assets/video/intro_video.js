/**
 * Stops and removes the video (used by the page if it needs to).
 */
export function close_intro() {
    wasm.close_intro();
}

/**
 * Test / tooling hook: "video_time,audio_time,paused,started,ready_state".
 * @returns {string}
 */
export function intro_debug_clock() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.intro_debug_clock(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Test / tooling hook: the clickable regions of the last frame as JSON ([{"a": "Contact", "x":..,"y":..,"w":..,"h":..}]).
 * @returns {string}
 */
export function intro_debug_hits() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.intro_debug_hits(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Test / tooling hook: put the pointer at (x, y) as if the mouse moved there.
 * @param {number} x
 * @param {number} y
 */
export function intro_debug_pointer(x, y) {
    wasm.intro_debug_pointer(x, y);
}

/**
 * Test / tooling hook: freeze the video clock at `t` seconds (the narration is paused).
 * @param {number} t
 */
export function intro_debug_time(t) {
    wasm.intro_debug_time(t);
}

/**
 * Called by the page when the play button is clicked: the button grows into the full-screen video.
 * @param {HTMLElement} button
 * @param {HTMLAudioElement} audio
 */
export function open_intro(button, audio) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.open_intro(retptr, addHeapObject(button), addHeapObject(audio));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_is_undefined_c05833b95a3cf397: function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        },
        __wbg___wbindgen_number_get_394265ed1e1b84ee: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_fffb441def202758: function(arg0) {
            getObject(arg0)._wbg_cb_unref();
        },
        __wbg_addColorStop_8eb1c8d584dae53b: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            getObject(arg0).addColorStop(arg1, getStringFromWasm0(arg2, arg3));
        }, arguments); },
        __wbg_addEventListener_d85450ee1320c989: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
        }, arguments); },
        __wbg_add_38cee25662852903: function() { return handleError(function (arg0, arg1, arg2) {
            getObject(arg0).add(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_appendChild_f553e8704c4f14a6: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_arc_61372d0a8f0a988c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).arc(arg1, arg2, arg3, arg4, arg5);
        }, arguments); },
        __wbg_beginPath_ca2dfce389ff20d2: function(arg0) {
            getObject(arg0).beginPath();
        },
        __wbg_bezierCurveTo_cb0279ca0ba5b76f: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            getObject(arg0).bezierCurveTo(arg1, arg2, arg3, arg4, arg5, arg6);
        },
        __wbg_body_40ec34e0a2931fe8: function(arg0) {
            const ret = getObject(arg0).body;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_cancelAnimationFrame_086d6084925c4e06: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).cancelAnimationFrame(arg1);
        }, arguments); },
        __wbg_catch_c1a60df4c30d76d3: function(arg0, arg1) {
            const ret = getObject(arg0).catch(getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_classList_8c12288eeff7eadb: function(arg0) {
            const ret = getObject(arg0).classList;
            return addHeapObject(ret);
        },
        __wbg_clientX_a7dcb4081126cd4b: function(arg0) {
            const ret = getObject(arg0).clientX;
            return ret;
        },
        __wbg_clientY_c0560910b20ee192: function(arg0) {
            const ret = getObject(arg0).clientY;
            return ret;
        },
        __wbg_clip_26d9280631a1d13e: function(arg0) {
            getObject(arg0).clip();
        },
        __wbg_closePath_0e752092e41e1e22: function(arg0) {
            getObject(arg0).closePath();
        },
        __wbg_createElement_fcbc0805de826d62: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_createRadialGradient_f08708ebb7199d95: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            const ret = getObject(arg0).createRadialGradient(arg1, arg2, arg3, arg4, arg5, arg6);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_currentTime_bf4477d7ec91feea: function(arg0) {
            const ret = getObject(arg0).currentTime;
            return ret;
        },
        __wbg_devicePixelRatio_1c0e0ed7deb19cd8: function(arg0) {
            const ret = getObject(arg0).devicePixelRatio;
            return ret;
        },
        __wbg_document_179650d6cb13c263: function(arg0) {
            const ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_drawImage_da8fc3deda491cc8: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).drawImage(getObject(arg1), arg2, arg3, arg4, arg5);
        }, arguments); },
        __wbg_ellipse_06527522251a4849: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
            getObject(arg0).ellipse(arg1, arg2, arg3, arg4, arg5, arg6, arg7);
        }, arguments); },
        __wbg_ended_4059297a4eabe5a6: function(arg0) {
            const ret = getObject(arg0).ended;
            return ret;
        },
        __wbg_error_744744ff0c9861e6: function(arg0) {
            console.error(getObject(arg0));
        },
        __wbg_fillRect_97b1f503e30148c3: function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
        },
        __wbg_fillText_e462ba58cec15054: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
        }, arguments); },
        __wbg_fill_7e2406c195723006: function(arg0) {
            getObject(arg0).fill();
        },
        __wbg_focus_2f77051f98540625: function() { return handleError(function (arg0) {
            getObject(arg0).focus();
        }, arguments); },
        __wbg_getBoundingClientRect_e828e6c31c66dea6: function(arg0) {
            const ret = getObject(arg0).getBoundingClientRect();
            return addHeapObject(ret);
        },
        __wbg_getContext_e79ddf6a9cb3cc76: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments); },
        __wbg_getElementById_1cbd8f06dbe8eb8e: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_height_96c07d9559d0200a: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_innerHeight_92315939e482496d: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).innerHeight;
            return addHeapObject(ret);
        }, arguments); },
        __wbg_innerWidth_dec7d2ac73df3e63: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).innerWidth;
            return addHeapObject(ret);
        }, arguments); },
        __wbg_instanceof_CanvasRenderingContext2d_2284b703b7023dcc: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlCanvasElement_ed02ed9136056019: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLCanvasElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_KeyboardEvent_be49f2d8e15d587a: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof KeyboardEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_PointerEvent_8ef1feb51407c0ed: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof PointerEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_05ba1ee4f6781663: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_key_803dca86cdcfa8dd: function(arg0, arg1) {
            const ret = getObject(arg1).key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_left_7e76a74d0db1754f: function(arg0) {
            const ret = getObject(arg0).left;
            return ret;
        },
        __wbg_lineTo_1aeefd30328165b5: function(arg0, arg1, arg2) {
            getObject(arg0).lineTo(arg1, arg2);
        },
        __wbg_matchMedia_9968278b31706f78: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).matchMedia(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments); },
        __wbg_matches_978994974df1e85b: function(arg0) {
            const ret = getObject(arg0).matches;
            return ret;
        },
        __wbg_measureText_c54a480a20a73a31: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).measureText(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_moveTo_2618bed6b5b25622: function(arg0, arg1, arg2) {
            getObject(arg0).moveTo(arg1, arg2);
        },
        __wbg_new_32b398fb48b6d94a: function() {
            const ret = new Array();
            return addHeapObject(ret);
        },
        __wbg_now_390768da5ee9e776: function(arg0) {
            const ret = getObject(arg0).now();
            return ret;
        },
        __wbg_of_5f1b88183ddb5d94: function(arg0, arg1) {
            const ret = Array.of(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_open_1a1b29d8bfde6885: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            const ret = getObject(arg0).open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), getStringFromWasm0(arg5, arg6));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments); },
        __wbg_pause_d5c0ce7f72f1a9b3: function() { return handleError(function (arg0) {
            getObject(arg0).pause();
        }, arguments); },
        __wbg_paused_b81c58e8c121d3fa: function(arg0) {
            const ret = getObject(arg0).paused;
            return ret;
        },
        __wbg_performance_3ef602e13d6c3b56: function(arg0) {
            const ret = getObject(arg0).performance;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_play_8708f9438e2fc4d9: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).play();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_pointerId_ea33d2695be12e7f: function(arg0) {
            const ret = getObject(arg0).pointerId;
            return ret;
        },
        __wbg_preventDefault_b64888c857500682: function(arg0) {
            getObject(arg0).preventDefault();
        },
        __wbg_quadraticCurveTo_c461b48b1c826818: function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).quadraticCurveTo(arg1, arg2, arg3, arg4);
        },
        __wbg_readyState_23bbbe8de6a4c92a: function(arg0) {
            const ret = getObject(arg0).readyState;
            return ret;
        },
        __wbg_rect_cbf18d19ffd5b10a: function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).rect(arg1, arg2, arg3, arg4);
        },
        __wbg_releasePointerCapture_3e982a4a25bf65a8: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).releasePointerCapture(arg1);
        }, arguments); },
        __wbg_removeEventListener_a3f23c70077bdcc1: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
        }, arguments); },
        __wbg_removeProperty_70da952bc1b493fa: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg1).removeProperty(getStringFromWasm0(arg2, arg3));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_remove_281d6b5594fade8f: function() { return handleError(function (arg0, arg1, arg2) {
            getObject(arg0).remove(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_remove_ce1b54059317fe8a: function(arg0) {
            getObject(arg0).remove();
        },
        __wbg_requestAnimationFrame_1a85deeab66448c2: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
            return ret;
        }, arguments); },
        __wbg_restore_ab535bc88702bcc0: function(arg0) {
            getObject(arg0).restore();
        },
        __wbg_rotate_6a6e81bc63bce7d8: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).rotate(arg1);
        }, arguments); },
        __wbg_roundRect_bed26ad5288a2716: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).roundRect(arg1, arg2, arg3, arg4, arg5);
        }, arguments); },
        __wbg_save_cd0bc920468bfe2c: function(arg0) {
            getObject(arg0).save();
        },
        __wbg_scale_c1f36b23e39bf061: function() { return handleError(function (arg0, arg1, arg2) {
            getObject(arg0).scale(arg1, arg2);
        }, arguments); },
        __wbg_scrollIntoView_5262843155744b69: function(arg0) {
            getObject(arg0).scrollIntoView();
        },
        __wbg_setAttribute_71039043be82d098: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setLineDash_63ce60143e4d578a: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).setLineDash(getObject(arg1));
        }, arguments); },
        __wbg_setPointerCapture_70025ca3fb7f26b9: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).setPointerCapture(arg1);
        }, arguments); },
        __wbg_setProperty_e4e51b1b1d681d15: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setTransform_d3001e44d696c566: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            getObject(arg0).setTransform(arg1, arg2, arg3, arg4, arg5, arg6);
        }, arguments); },
        __wbg_set_className_e0b1e805ac9ecbf4: function(arg0, arg1, arg2) {
            getObject(arg0).className = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_currentTime_dfb207a006f1af24: function(arg0, arg1) {
            getObject(arg0).currentTime = arg1;
        },
        __wbg_set_fillStyle_4360b989b9352bbb: function(arg0, arg1, arg2) {
            getObject(arg0).fillStyle = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_fillStyle_ed3ce34a55db5220: function(arg0, arg1) {
            getObject(arg0).fillStyle = getObject(arg1);
        },
        __wbg_set_font_33fee74f2c82cb6f: function(arg0, arg1, arg2) {
            getObject(arg0).font = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_globalAlpha_9b3de2f2aa9958de: function(arg0, arg1) {
            getObject(arg0).globalAlpha = arg1;
        },
        __wbg_set_height_7d9d8f892e6964c6: function(arg0, arg1) {
            getObject(arg0).height = arg1 >>> 0;
        },
        __wbg_set_lineCap_2b3a8b4e4499222b: function(arg0, arg1, arg2) {
            getObject(arg0).lineCap = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_lineJoin_f7a0158da26951b7: function(arg0, arg1, arg2) {
            getObject(arg0).lineJoin = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_lineWidth_beb3d05e36f4cc53: function(arg0, arg1) {
            getObject(arg0).lineWidth = arg1;
        },
        __wbg_set_muted_d064b10668f36535: function(arg0, arg1) {
            getObject(arg0).muted = arg1 !== 0;
        },
        __wbg_set_strokeStyle_b390d5f09a6989a8: function(arg0, arg1, arg2) {
            getObject(arg0).strokeStyle = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_textAlign_75f93b22c0415d5d: function(arg0, arg1, arg2) {
            getObject(arg0).textAlign = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_textBaseline_edb08ba62ac0d3ac: function(arg0, arg1, arg2) {
            getObject(arg0).textBaseline = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_width_8e30d010cd66830d: function(arg0, arg1) {
            getObject(arg0).width = arg1 >>> 0;
        },
        __wbg_shiftKey_9bcb8bdd60c2f152: function(arg0) {
            const ret = getObject(arg0).shiftKey;
            return ret;
        },
        __wbg_static_accessor_GLOBAL_4ef717fb391d88b7: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_8d1badc68b5a74f4: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_SELF_146583524fe1469b: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_WINDOW_f2829a2234d7819e: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_stroke_cf809e69aae41b03: function(arg0) {
            getObject(arg0).stroke();
        },
        __wbg_style_6657aed849e5d757: function(arg0) {
            const ret = getObject(arg0).style;
            return addHeapObject(ret);
        },
        __wbg_top_fe120acfa924a430: function(arg0) {
            const ret = getObject(arg0).top;
            return ret;
        },
        __wbg_translate_d2b84d406c25580d: function() { return handleError(function (arg0, arg1, arg2) {
            getObject(arg0).translate(arg1, arg2);
        }, arguments); },
        __wbg_warn_b1370d804fa3e259: function(arg0) {
            console.warn(getObject(arg0));
        },
        __wbg_width_219185400361db86: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbg_width_84477c442af415ce: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 4, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_199);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [F64], shim_idx: 3, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_200);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000003: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
    };
    return {
        __proto__: null,
        "./intro_video_bg.js": import0,
    };
}

function __wasm_bindgen_func_elem_199(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_199(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_200(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_200(arg0, arg1, arg2);
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_export4(state.a, state.b));

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export(addHeapObject(e));
    }
}

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_export4(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('intro_video_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
