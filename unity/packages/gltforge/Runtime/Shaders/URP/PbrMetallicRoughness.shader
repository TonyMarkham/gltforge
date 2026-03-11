// GLTF/URP/PbrMetallicRoughness
// glTF 2.0 PBR metallic-roughness, written for the Universal Render Pipeline.
//
// Key difference from Unity's built-in URP/Lit:
//   The metallic-roughness texture uses glTF channel packing —
//     G = perceptual roughness   (NOT smoothness)
//     B = metallic
//   — rather than Unity's R=metallic / A=smoothness layout.
//   Conversion to smoothness (1 - roughness) is done in the fragment shader,
//   so textures can be used directly without any CPU swizzle.
//
// Requires URP 10 or later (SurfaceData.clearCoatMask / clearCoatSmoothness).

Shader "GLTF/URP/PbrMetallicRoughness"
{
    Properties
    {
        // ── Base color ───────────────────────────────────────────────────────
        _BaseColor          ("Base Color Factor",          Color)        = (1,1,1,1)
        _BaseMap            ("Base Color Texture",         2D)           = "white" {}

        // ── Metallic-roughness (glTF packing: G = roughness, B = metallic) ──
        _MetallicRoughnessMap ("Metal Rough Texture",      2D)           = "white" {}
        _Metallic           ("Metallic Factor",            Range(0,1))   = 1.0
        _Roughness          ("Roughness Factor",           Range(0,1))   = 1.0

        // ── Normal map ───────────────────────────────────────────────────────
        _BumpMap            ("Normal Map",                 2D)           = "bump"  {}
        _BumpScale          ("Normal Scale",               Float)        = 1.0

        // ── Occlusion ────────────────────────────────────────────────────────
        _OcclusionMap       ("Occlusion Texture",          2D)           = "white" {}
        _OcclusionStrength  ("Occlusion Strength",         Range(0,1))   = 1.0

        // ── Emission ─────────────────────────────────────────────────────────
        [HDR] _EmissionColor ("Emissive Factor",           Color)        = (0,0,0,1)
        _EmissionMap        ("Emissive Texture",           2D)           = "black" {}

        // ── Alpha ────────────────────────────────────────────────────────────
        _Cutoff             ("Alpha Cutoff",               Range(0,1))   = 0.5

        // ── Rendering mode (set by importer, hidden from Inspector) ──────────
        // _Surface: 0 = Opaque, 1 = Transparent
        [HideInInspector] _Surface      ("__surface",  Float) = 0.0
        [HideInInspector] _Cull         ("__cull",     Float) = 2.0
        [HideInInspector] _SrcBlend     ("__src",      Float) = 1.0
        [HideInInspector] _DstBlend     ("__dst",      Float) = 0.0
        [HideInInspector] _ZWrite       ("__zw",       Float) = 1.0
    }

    SubShader
    {
        Tags
        {
            "RenderType"            = "Opaque"
            "RenderPipeline"        = "UniversalPipeline"
            "UniversalMaterialType" = "Lit"
            "IgnoreProjector"       = "True"
        }
        LOD 300

        // ── Forward Lit ───────────────────────────────────────────────────────
        Pass
        {
            Name "ForwardLit"
            Tags { "LightMode" = "UniversalForward" }

            Blend   [_SrcBlend] [_DstBlend]
            ZWrite  [_ZWrite]
            Cull    [_Cull]

            HLSLPROGRAM
            #pragma target 2.0

            // Per-material feature keywords — enabled by the importer when slots are present.
            #pragma shader_feature_local_fragment _ALPHATEST_ON
            #pragma shader_feature_local_fragment _SURFACE_TYPE_TRANSPARENT
            #pragma shader_feature_local_fragment _NORMALMAP
            #pragma shader_feature_local_fragment _METALLICROUGHNESSMAP
            #pragma shader_feature_local_fragment _OCCLUSIONMAP
            #pragma shader_feature_local_fragment _EMISSION

            // URP required multi-compiles.
            #pragma multi_compile _ _MAIN_LIGHT_SHADOWS _MAIN_LIGHT_SHADOWS_CASCADE _MAIN_LIGHT_SHADOWS_SCREEN
            #pragma multi_compile _ _ADDITIONAL_LIGHTS_VERTEX _ADDITIONAL_LIGHTS
            #pragma multi_compile_fragment _ _ADDITIONAL_LIGHT_SHADOWS
            #pragma multi_compile_fragment _ _REFLECTION_PROBE_BLENDING
            #pragma multi_compile_fragment _ _REFLECTION_PROBE_BOX_PROJECTION
            #pragma multi_compile_fragment _ _SHADOWS_SOFT
            #pragma multi_compile_fragment _ _SCREEN_SPACE_OCCLUSION
            #pragma multi_compile_fragment _ _DBUFFER_MRT1 _DBUFFER_MRT2 _DBUFFER_MRT3
            #pragma multi_compile_fragment _ _LIGHT_LAYERS
            #pragma multi_compile_fragment _ _LIGHT_COOKIES
            #pragma multi_compile _ _CLUSTERED_RENDERING
            #pragma multi_compile _ LIGHTMAP_ON
            #pragma multi_compile _ DYNAMICLIGHTMAP_ON
            #pragma multi_compile _ DIRLIGHTMAP_COMBINED
            #pragma multi_compile_fog
            #pragma multi_compile_instancing

            #pragma vertex   ForwardVert
            #pragma fragment ForwardFrag

            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Core.hlsl"
            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Lighting.hlsl"
            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/SurfaceInput.hlsl"

            // ── Uniform declarations ─────────────────────────────────────────

            CBUFFER_START(UnityPerMaterial)
                float4 _BaseMap_ST;
                half4  _BaseColor;
                half   _Metallic;
                half   _Roughness;
                half   _BumpScale;
                half   _OcclusionStrength;
                half4  _EmissionColor;
                half   _Cutoff;
            CBUFFER_END

            // _BaseMap, _BumpMap, _EmissionMap are declared by SurfaceInput.hlsl above.
            TEXTURE2D(_MetallicRoughnessMap); SAMPLER(sampler_MetallicRoughnessMap);
            TEXTURE2D(_OcclusionMap);         SAMPLER(sampler_OcclusionMap);

            // ── Vertex I/O ───────────────────────────────────────────────────

            struct Attributes
            {
                float4 positionOS           : POSITION;
                float3 normalOS             : NORMAL;
                float4 tangentOS            : TANGENT;
                float2 uv                   : TEXCOORD0;
                float2 staticLightmapUV     : TEXCOORD1;
                UNITY_VERTEX_INPUT_INSTANCE_ID
            };

            struct Varyings
            {
                float4 positionCS           : SV_POSITION;
                float2 uv                   : TEXCOORD0;
                float3 positionWS           : TEXCOORD1;
                float3 normalWS             : TEXCOORD2;
                #ifdef _NORMALMAP
                half4  tangentWS            : TEXCOORD3;  // xyz = tangent, w = sign
                #endif
                half3  viewDirWS            : TEXCOORD4;
                DECLARE_LIGHTMAP_OR_SH(staticLightmapUV, vertexSH, 5);
                half4  fogFactorAndVertexLight : TEXCOORD6; // x = fog, yzw = vertex light
                UNITY_VERTEX_INPUT_INSTANCE_ID
                UNITY_VERTEX_OUTPUT_STEREO
            };

            // ── Vertex shader ────────────────────────────────────────────────

            Varyings ForwardVert(Attributes IN)
            {
                Varyings OUT = (Varyings)0;
                UNITY_SETUP_INSTANCE_ID(IN);
                UNITY_TRANSFER_INSTANCE_ID(IN, OUT);
                UNITY_INITIALIZE_VERTEX_OUTPUT_STEREO(OUT);

                VertexPositionInputs posInputs = GetVertexPositionInputs(IN.positionOS.xyz);
                VertexNormalInputs   nrmInputs = GetVertexNormalInputs(IN.normalOS, IN.tangentOS);

                OUT.positionCS = posInputs.positionCS;
                OUT.positionWS = posInputs.positionWS;
                OUT.uv         = TRANSFORM_TEX(IN.uv, _BaseMap);
                OUT.normalWS   = nrmInputs.normalWS;

                #ifdef _NORMALMAP
                real sign         = IN.tangentOS.w * GetOddNegativeScale();
                OUT.tangentWS     = half4(nrmInputs.tangentWS, sign);
                #endif

                OUT.viewDirWS = GetWorldSpaceViewDir(posInputs.positionWS);

                OUTPUT_LIGHTMAP_UV(IN.staticLightmapUV, unity_LightmapST, OUT.staticLightmapUV);
                OUTPUT_SH(OUT.normalWS.xyz, OUT.vertexSH);

                half fogFactor    = ComputeFogFactor(posInputs.positionCS.z);
                half3 vertexLight = VertexLighting(posInputs.positionWS, nrmInputs.normalWS);
                OUT.fogFactorAndVertexLight = half4(fogFactor, vertexLight);

                return OUT;
            }

            // ── Fragment shader ──────────────────────────────────────────────

            half4 ForwardFrag(Varyings IN) : SV_Target
            {
                UNITY_SETUP_INSTANCE_ID(IN);
                UNITY_SETUP_STEREO_EYE_INDEX_POST_VERTEX(IN);

                // Base color + alpha.
                half4 albedo = SAMPLE_TEXTURE2D(_BaseMap, sampler_BaseMap, IN.uv) * _BaseColor;

                #ifdef _ALPHATEST_ON
                    clip(albedo.a - _Cutoff);
                #endif

                // Metallic-roughness — glTF packing: G = roughness, B = metallic.
                half metallic           = _Metallic;
                half perceptualRoughness = _Roughness;
                #ifdef _METALLICROUGHNESSMAP
                    half4 mr = SAMPLE_TEXTURE2D(_MetallicRoughnessMap, sampler_MetallicRoughnessMap, IN.uv);
                    metallic            *= mr.b;
                    perceptualRoughness *= mr.g;
                #endif
                // glTF perceptual roughness → URP perceptual smoothness.
                half smoothness = 1.0h - perceptualRoughness;

                // Normal map.
                half3 normalTS = half3(0.0h, 0.0h, 1.0h);
                #ifdef _NORMALMAP
                    normalTS = UnpackNormalScale(
                        SAMPLE_TEXTURE2D(_BumpMap, sampler_BumpMap, IN.uv), _BumpScale);
                #endif

                // Occlusion — glTF stores AO in the R channel.
                half occlusion = 1.0h;
                #ifdef _OCCLUSIONMAP
                    occlusion = LerpWhiteTo(
                        SAMPLE_TEXTURE2D(_OcclusionMap, sampler_OcclusionMap, IN.uv).r,
                        _OcclusionStrength);
                #endif

                // Emission.
                half3 emission = _EmissionColor.rgb;
                #ifdef _EMISSION
                    emission *= SAMPLE_TEXTURE2D(_EmissionMap, sampler_EmissionMap, IN.uv).rgb;
                #endif

                // Build URP SurfaceData.
                SurfaceData surface         = (SurfaceData)0;
                surface.albedo              = albedo.rgb;
                surface.metallic            = metallic;
                surface.specular            = half3(0, 0, 0);
                surface.smoothness          = smoothness;
                surface.normalTS            = normalTS;
                surface.emission            = emission;
                surface.occlusion           = occlusion;
                surface.alpha               = albedo.a;
                surface.clearCoatMask       = 0;
                surface.clearCoatSmoothness = 0;

                // Build URP InputData.
                InputData inputData         = (InputData)0;
                inputData.positionWS        = IN.positionWS;
                inputData.positionCS        = IN.positionCS;
                inputData.viewDirectionWS   = SafeNormalize(IN.viewDirWS);

                #ifdef _NORMALMAP
                    float  sign     = IN.tangentWS.w;
                    float3 bitangent = sign * cross(IN.normalWS.xyz, IN.tangentWS.xyz);
                    inputData.normalWS = TransformTangentToWorld(normalTS,
                        float3x3(IN.tangentWS.xyz, bitangent, IN.normalWS.xyz));
                #else
                    inputData.normalWS = IN.normalWS;
                #endif
                inputData.normalWS = NormalizeNormalPerPixel(inputData.normalWS);

                inputData.fogCoord              = InitializeInputDataFog(float4(IN.positionWS, 1.0), IN.fogFactorAndVertexLight.x);
                inputData.vertexLighting        = IN.fogFactorAndVertexLight.yzw;
                inputData.bakedGI               = SAMPLE_GI(IN.staticLightmapUV, IN.vertexSH, inputData.normalWS);
                inputData.normalizedScreenSpaceUV = GetNormalizedScreenSpaceUV(IN.positionCS);
                inputData.shadowMask            = SAMPLE_SHADOWMASK(IN.staticLightmapUV);

                half4 color = UniversalFragmentPBR(inputData, surface);
                color.rgb   = MixFog(color.rgb, inputData.fogCoord);

                #ifdef _SURFACE_TYPE_TRANSPARENT
                    color.a = albedo.a;
                #else
                    color.a = 1.0h;
                #endif

                return color;
            }
            ENDHLSL
        }

        // ── Shadow Caster ─────────────────────────────────────────────────────
        Pass
        {
            Name "ShadowCaster"
            Tags { "LightMode" = "ShadowCaster" }

            ZWrite On
            ZTest  LEqual
            ColorMask 0
            Cull [_Cull]

            HLSLPROGRAM
            #pragma target 2.0
            #pragma shader_feature_local_fragment _ALPHATEST_ON
            #pragma multi_compile_vertex _ _CASTING_PUNCTUAL_LIGHT_SHADOW
            #pragma multi_compile_instancing

            #pragma vertex   ShadowPassVertex
            #pragma fragment ShadowPassFragment

            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Core.hlsl"

            CBUFFER_START(UnityPerMaterial)
                float4 _BaseMap_ST;
                half4  _BaseColor;
                half   _Metallic;
                half   _Roughness;
                half   _BumpScale;
                half   _OcclusionStrength;
                half4  _EmissionColor;
                half   _Cutoff;
            CBUFFER_END

            TEXTURE2D(_BaseMap); SAMPLER(sampler_BaseMap);

            #include "Packages/com.unity.render-pipelines.universal/Shaders/ShadowCasterPass.hlsl"
            ENDHLSL
        }

        // ── Depth Only ────────────────────────────────────────────────────────
        Pass
        {
            Name "DepthOnly"
            Tags { "LightMode" = "DepthOnly" }

            ZWrite On
            ColorMask 0
            Cull [_Cull]

            HLSLPROGRAM
            #pragma target 2.0
            #pragma shader_feature_local_fragment _ALPHATEST_ON
            #pragma multi_compile_instancing

            #pragma vertex   DepthOnlyVertex
            #pragma fragment DepthOnlyFragment

            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Core.hlsl"

            CBUFFER_START(UnityPerMaterial)
                float4 _BaseMap_ST;
                half4  _BaseColor;
                half   _Metallic;
                half   _Roughness;
                half   _BumpScale;
                half   _OcclusionStrength;
                half4  _EmissionColor;
                half   _Cutoff;
            CBUFFER_END

            TEXTURE2D(_BaseMap); SAMPLER(sampler_BaseMap);

            #include "Packages/com.unity.render-pipelines.universal/Shaders/DepthOnlyPass.hlsl"
            ENDHLSL
        }

        // ── Depth Normals ─────────────────────────────────────────────────────
        Pass
        {
            Name "DepthNormals"
            Tags { "LightMode" = "DepthNormals" }

            ZWrite On
            Cull [_Cull]

            HLSLPROGRAM
            #pragma target 2.0
            #pragma shader_feature_local_fragment _ALPHATEST_ON
            #pragma shader_feature_local _NORMALMAP
            #pragma multi_compile_instancing

            #pragma vertex   DepthNormalsVertex
            #pragma fragment DepthNormalsFragment

            #include "Packages/com.unity.render-pipelines.universal/ShaderLibrary/Core.hlsl"

            CBUFFER_START(UnityPerMaterial)
                float4 _BaseMap_ST;
                half4  _BaseColor;
                half   _Metallic;
                half   _Roughness;
                half   _BumpScale;
                half   _OcclusionStrength;
                half4  _EmissionColor;
                half   _Cutoff;
            CBUFFER_END

            TEXTURE2D(_BaseMap); SAMPLER(sampler_BaseMap);
            TEXTURE2D(_BumpMap); SAMPLER(sampler_BumpMap);

            #include "Packages/com.unity.render-pipelines.universal/Shaders/DepthNormalsPass.hlsl"
            ENDHLSL
        }
    }

    FallBack "Hidden/Universal Render Pipeline/FallbackError"
}
