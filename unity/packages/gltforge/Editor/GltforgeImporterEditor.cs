using UnityEditor;
using UnityEditor.AssetImporters;
using UnityEditor.UIElements;
using UnityEngine;
using UnityEngine.UIElements;

namespace Gltforge.Editor
{
    [CustomEditor(typeof(GltforgeImporter))]
    public class GltforgeImporterEditor : ScriptedImporterEditor
    {
        [SerializeField] VisualTreeAsset _layout;

        public override VisualElement CreateInspectorGUI()
        {
            var root = _layout.CloneTree();
            root.Bind(serializedObject);
            root.Add(new IMGUIContainer(ApplyRevertGUI));
            return root;
        }
    }
}
