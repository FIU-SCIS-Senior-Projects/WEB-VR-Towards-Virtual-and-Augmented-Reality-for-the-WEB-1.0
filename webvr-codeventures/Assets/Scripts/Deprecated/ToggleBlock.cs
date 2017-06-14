using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ToggleBlock : MonoBehaviour {

    public bool animate;
    public bool active;

	// Use this for initialization
	void Start ()
    {
        InvokeRepeating("toggleActive", 0, 1.0f);
	}

    private void toggleActive()
    {
        if(animate) active = !active;
        gameObject.SetActive(active);
    }
}
