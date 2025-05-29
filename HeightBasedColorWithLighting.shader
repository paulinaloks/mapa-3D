Shader "Custom/HeightBasedColorWithLighting" //ChatGPT 
{
    Properties
    {
        _MinHeight ("Minimum Height", Float) = 100
        _MaxHeight ("Maximum Height", Float) = 500
        _BaseColor ("Base Color", Color) = (0, 1, 0, 1)  
        _LightColor ("Light Color", Color) = (1, 1, 1, 1) 
    }
    SubShader
    {
        Tags { "RenderType"="Opaque" }
        LOD 100

        Pass
        { 
            CGPROGRAM
            #pragma vertex vert
            #pragma fragment frag

            #include "UnityCG.cginc"

            struct appdata_t
            {
                float4 vertex : POSITION;
                float3 normal : NORMAL;
            };

            struct v2f
            {
                float4 pos : SV_POSITION;
                float height : TEXCOORD0;
                float3 normal : TEXCOORD1;
            };

            float _MinHeight;
            float _MaxHeight;
            fixed4 _BaseColor;
            fixed4 _LightColor;

            
            v2f vert(appdata_t v)
            {
                v2f o;
                o.pos = UnityObjectToClipPos(v.vertex);
                o.height = (v.vertex.y - _MinHeight) / (_MaxHeight - _MinHeight);
                o.normal = normalize(v.normal);
                return o;
            }

            fixed4 frag(v2f i) : SV_Target
            {
                float t = saturate(i.height); 
                fixed4 terrainColor = lerp(fixed4(0, 1, 0, 1), fixed4(1, 0, 0, 1), t); 
                float3 lightDir = normalize(float3(0.5, 1, 0.5));  
                float diffuse = max(0, dot(i.normal, lightDir));  

                fixed4 finalColor = terrainColor * _LightColor * diffuse;

                return finalColor;
            }
            ENDCG
        }
    }

    FallBack "Diffuse"
}
