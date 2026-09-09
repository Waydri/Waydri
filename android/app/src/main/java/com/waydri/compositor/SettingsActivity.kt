package com.waydri.compositor

import android.os.Bundle
import android.widget.*
import androidx.appcompat.app.AppCompatActivity

class SettingsActivity : AppCompatActivity() {
    private lateinit var viewModel: SettingsViewModel

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        viewModel = SettingsViewModel(this)

        val scroll = ScrollView(this)
        val layout = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }

        layout.addView(sectionLabel("Display"))
        layout.addView(switchRow("Vsync", viewModel.getBool("vsync", true)) { viewModel.setBool("vsync", it) })
        layout.addView(switchRow("Blur", viewModel.getBool("blur", false)) { viewModel.setBool("blur", it) })
        layout.addView(switchRow("Shadows", viewModel.getBool("shadows", false)) { viewModel.setBool("shadows", it) })
        layout.addView(sliderRow("Opacity", viewModel.getFloat("opacity", 1.0f)) { viewModel.setFloat("opacity", it) })

        layout.addView(sectionLabel("Layout"))
        layout.addView(spinnerRow("Default Layout", viewModel.getString("layout", "master_stack"),
            listOf("master_stack", "grid", "dwindle")) { viewModel.setString("layout", it) })

        layout.addView(sectionLabel("Performance"))
        layout.addView(sliderRow("Max FPS", viewModel.getInt("max_fps", 60).toFloat(),
            min = 30f, max = 240f) { viewModel.setInt("max_fps", it.toInt()) })
        layout.addView(switchRow("Animations", viewModel.getBool("animations", true)) { viewModel.setBool("animations", it) })

        scroll.addView(layout)
        setContentView(scroll)
    }

    private fun sectionLabel(text: String): TextView {
        return TextView(this).apply {
            this.text = text
            setPadding(32, 32, 16, 8)
            textSize = 18f
            setTextColor(resources.getColor(android.R.color.holo_blue_light, theme))
        }
    }

    private fun switchRow(label: String, initial: Boolean, onChange: (Boolean) -> Unit): LinearLayout {
        val row = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            setPadding(32, 16, 32, 16)
        }
        row.addView(TextView(this).apply {
            this.text = label
            layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        })
        val toggle = Switch(this).apply {
            isChecked = initial
            setOnCheckedChangeListener { _, checked -> onChange(checked) }
        }
        row.addView(toggle)
        return row
    }

    private fun spinnerRow(label: String, current: String, options: List<String>, onSelect: (String) -> Unit): LinearLayout {
        val row = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            setPadding(32, 16, 32, 16)
        }
        row.addView(TextView(this).apply {
            this.text = label
            layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
        })
        val spinner = Spinner(this).apply {
            adapter = ArrayAdapter(this@SettingsActivity, android.R.layout.simple_spinner_dropdown_item, options)
            setSelection(options.indexOf(current).coerceAtLeast(0))
            onItemSelectedListener = object : AdapterView.OnItemSelectedListener {
                override fun onItemSelected(parent: AdapterView<*>?, view: android.view.View?, pos: Int, id: Long) {
                    onSelect(options[pos])
                }
                override fun onNothingSelected(parent: AdapterView<*>?) {}
            }
        }
        row.addView(spinner)
        return row
    }

    private fun sliderRow(label: String, initial: Float, min: Float = 0f, max: Float = 1f,
                          onChange: (Float) -> Unit): LinearLayout {
        val row = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(32, 16, 32, 16)
        }
        val valueText = TextView(this).apply {
            text = "$label: ${"%.1f".format(initial)}"
        }
        row.addView(valueText)
        val slider = SeekBar(this).apply {
            max = ((max - min) * 100).toInt()
            progress = ((initial - min) * 100).toInt()
            setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                override fun onProgressChanged(sb: SeekBar?, progress: Int, fromUser: Boolean) {
                    val value = min + progress / 100f
                    valueText.text = "$label: ${"%.1f".format(value)}"
                    if (fromUser) onChange(value)
                }
                override fun onStartTrackingTouch(sb: SeekBar?) {}
                override fun onStopTrackingTouch(sb: SeekBar?) {}
            })
        }
        row.addView(slider)
        return row
    }
}
