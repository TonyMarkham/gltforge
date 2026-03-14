using UnityEngine;

namespace Gltforge
{
    [CreateAssetMenu(fileName = "ExportGltf", menuName = "Scriptable Objects/ExportGltf")]
    public class ExportGltf : ScriptableObject
    {
        [SerializeField] private GameObject target;

        [ContextMenu("Export", isValidateFunction: false)]
        private void Export()
        {
            
        }

        [ContextMenu("Export", isValidateFunction: true)]
        private bool Export_validation()
        {
            return target != null;
        }
    }
}
