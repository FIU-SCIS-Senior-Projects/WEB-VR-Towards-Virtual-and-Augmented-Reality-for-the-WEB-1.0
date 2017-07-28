using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class MovePlayer : MonoBehaviour
{
    public AudioClip bump;
    public AudioClip restart;

    private AudioSource audio;
    private Vector3 originalPos;
    private RaycastHit hit;

    private bool onStairs = false;

    // Use this for initialization
    void Start ()
    {
        originalPos = transform.position;
        audio = GetComponent<AudioSource>();
	}
	
	// Update is called once per frame
	void Update ()
    {
        if (Input.GetKeyDown("up")) upKeyPressed();
        if (Input.GetKeyDown("down")) downKeyPressed();
        if (Input.GetKeyDown("left")) leftKeyPressed();
        if (Input.GetKeyDown("right")) rightKeyPressed();
    }

    private void LateUpdate ()
    {
        // player is falling
        if (canMove("down") && !onStairs)
        {
            transform.Translate(new Vector3(0, 0, -Constants.MOVE_DISTANCE));
        }
        else if(canMove("down") && onStairs)
        {
            // do nothing
        }
        else
        {
            // player can't move, so he probably landed on bottom plane aka died
            if (hit.collider.tag == "Ground")
            {
                playSound(restart);
                transform.position = originalPos;
            }
        }
    }

    void playSound(AudioClip c)
    {
        print("Playing!");
        audio.PlayOneShot(c);
    }

    bool canMove(string dir)
    {
        Vector3 newPos;

        switch(dir)
        {
            case "forward": newPos = Vector3.forward; break;
            case "back": newPos = Vector3.back; break;
            case "left": newPos = Vector3.left; break;
            case "right": newPos = Vector3.right; break;
            case "down": newPos = Vector3.down; break;
            default: newPos = Vector3.zero; break;
        }

        Physics.Raycast(transform.position, newPos, out hit, Constants.MOVE_DISTANCE);

        return !hit.collider; // no collider = can move
    }

    void upKeyPressed ()
    {
        onStairs = false;

        if (canMove("forward"))
        {
            transform.Translate(new Vector3(0, -Constants.MOVE_DISTANCE));
        }
        else
        {
            if (hit.collider.tag == "Stair")
            {
                transform.Translate(new Vector3(0, 0, Constants.MOVE_DISTANCE));
                onStairs = true;
            }
            else
            {
                playSound(bump); // play 'bump' sound effect
            }
        }
    }

    void downKeyPressed()
    {
        onStairs = false;

        if (canMove("back"))
        {
            transform.Translate(new Vector3(0, Constants.MOVE_DISTANCE));
        }
        else
        {
            if (hit.collider.tag == "Stair")
            {
                onStairs = true;
                transform.Translate(new Vector3(0, 0, Constants.MOVE_DISTANCE));
            }
            else
            {
                playSound(bump); // play 'bump' sound effect
            }
        }
    }

    void leftKeyPressed()
    {
        onStairs = false;

        if (canMove("left"))
        {
            transform.Translate(new Vector3(-Constants.MOVE_DISTANCE, 0));
        }
        else
        {
            if (hit.collider.tag == "Stair")
            {
                transform.Translate(new Vector3(0, 0, Constants.MOVE_DISTANCE));
                onStairs = true;
            }
            else
            {
                playSound(bump); // play 'bump' sound effect
            }
        }
    }

    void rightKeyPressed()
    {
        onStairs = false;

        if (canMove("right"))
        {
            transform.Translate(new Vector3(Constants.MOVE_DISTANCE, 0));
        }
        else
        {
            if (hit.collider.tag == "Stair")
            {
                transform.Translate(new Vector3(0, 0, Constants.MOVE_DISTANCE));
                onStairs = true;
            }
            else
            {
                playSound(bump); // play 'bump' sound effect
            }
        }
    }
}
