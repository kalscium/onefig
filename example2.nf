conff toml stewart = "stewart.toml";

# dave = "something else" // should throw error cause it's assigned to in the other onefig-script

stewart.name: "Stewart Lee"
stewart.age = 30,
stewart.skills = [ "nothing :C" ]
stewart.dog.name: "Bob";
stewart.dog: {
    age = 2;
    children = {
        cheese: { etc: [[["that's alotta lists"]]] },
    }
}